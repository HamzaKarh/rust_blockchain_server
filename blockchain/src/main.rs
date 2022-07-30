#![feature(proc_macro_hygiene, decl_macro)]
use crate::blockchain::Blockchain;
use rocket::State;
use rocket::{self, get, routes};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, SystemTime};

const BLOCK_MINING_TIME: Duration = Duration::from_secs(10);

mod blockchain;

struct SendChannel {
    sender: Mutex<Sender<Vec<u128>>>,
    receiver: Mutex<Receiver<String>>,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/transfer/<sender>/<receiver>/<funds>")]
fn transfer(sender: u64, receiver: u64, funds: u128, shared: State<SendChannel>) -> &'static str {
    let lock = shared.sender.lock().expect("lock shared data");
    let action = handle_transfer(sender, receiver, funds);
    lock.send(action).unwrap();
    "Successfully sent transfer transaction to blockchain"
}

#[get("/balance/<id>")]
fn balance(id: u64, shared: State<SendChannel>) -> String {
    let lock_sender = shared.sender.lock().expect("lock shared data");
    let lock_receiver = shared.receiver.lock().expect("lock shared data");
    let action = handle_read_balance(id);
    lock_sender.send(action).unwrap();
    lock_receiver.recv().unwrap()
    // "Successfully sent read command to blockchain"
}

#[get("/create_account/<id>/<funds>")]
fn create_account(id: u64, funds: u128, shared: State<SendChannel>) -> &'static str {
    let lock = shared.sender.lock().expect("lock shared data");
    let action = handle_account_creation(id, funds);
    lock.send(action).unwrap();
    "Successfully sent account creation transaction to blockchain"
}

#[get("/start_node")]
fn start_node(shared: State<SendChannel>) -> &'static str {
    let lock = shared.sender.lock().expect("lock shared data");
    let mut action: Vec<u128> = Vec::new();
    action.push(0);
    lock.send(action).unwrap();
    "Node successfully started"
}

fn main() {
    let (send, recv) = channel();
    let (return_send, return_rcv) = channel::<String>();
    thread::spawn(move || {
        handle_server(send, return_rcv);
    });
    handle_blockchain(recv, return_send);
}

fn handle_server(sender: Sender<Vec<u128>>, return_rcv: Receiver<String>) {
    let send_channel = SendChannel {
        sender: Mutex::new(sender),
        receiver: Mutex::new(return_rcv),
    };
    rocket::ignite()
        .manage(send_channel)
        .mount(
            "/",
            routes![index, start_node, create_account, transfer, balance],
        )
        .launch();
}

fn handle_blockchain(recv: Receiver<Vec<u128>>, return_send: Sender<String>) {
    // let mut val = -1;
    let mut b = Blockchain::new();
    while !b.running {
        match recv.try_recv() {
            Ok(i) => {
                if i.first() == None {
                    continue;
                }
                if *i.first().unwrap() == 0 as u128 {
                    println!(" command number {}", *i.first().unwrap());
                    b.set_running(true);
                } else if *i.first().unwrap() == 4 as u128 {
                    println!("Exiting without starting the blockchain? Alright... bye!");
                    return;
                } else {
                    println!(" \n \n \n !!!! Error : Please start the blockchain first");
                }
            }
            Err(_) => {
                thread::sleep(Duration::from_millis(200));
                continue;
            }
        }
    }

    let start_time = SystemTime::now();
    while b.running {
        match recv.try_recv() {
            Ok(i) => {
                return_send.send(handle_commands(i, &mut b)).unwrap();
            }
            Err(_) => {}
        }
        if start_time.elapsed().unwrap().as_secs() % BLOCK_MINING_TIME.as_secs() < 1 {
            b.mine();
            thread::sleep(Duration::from_secs(1));
        }
    }

    println!("EXITING....");
}

fn handle_read_balance(id: u64) -> Vec<u128> {
    let mut action: Vec<u128> = Vec::new();
    action.push(3);
    action.push(id as u128);
    action
}

fn handle_account_creation(id: u64, funds: u128) -> Vec<u128> {
    let mut action = Vec::new();
    action.push(1);
    action.push(id as u128);
    action.push(funds);
    action
}

fn handle_transfer(sender: u64, receiver: u64, funds: u128) -> Vec<u128> {
    let mut action: Vec<u128> = Vec::new();
    action.push(2);
    action.push(sender as u128);
    action.push(receiver as u128);
    action.push(funds);
    action
}

fn handle_commands(commands: Vec<u128>, b: &mut Blockchain) -> String {
    match commands[0] {
        1 => b.create_account(commands[1] as u64, commands[2] as u128),

        2 => b.transfer(commands[1] as u64, commands[2] as u64, commands[3] as u128),

        3 => {
            let balance = b.read_balance(commands[1] as u64);
            if balance < 0 {
                return "Account does not exist".to_string();
            }
            return "balance for account ".to_owned()
                + commands[1].to_string().as_str()
                + " : "
                + &*balance.to_string();
        }

        4 => b.set_running(false),

        _ => println!(
            "Error: an unknown command slipped through the cracks command : {}",
            commands[0]
        ),
    }
    "ok".to_string()
}
