use std::error::Error;
use std::io;
use std::process::Command;
use std::time::Duration;
use std::{env, fmt, thread};

#[derive(Debug)]
struct MyError(String);
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for MyError {}

const BASE_URL: &str = "http://localhost:8000";

fn start_server() {
    let current_dir = env::current_dir();
    let path = current_dir
        .unwrap()
        .display()
        .to_string()
        .replace("console", "blockchain");
    let cmd = "gnome-terminal -- bash -c 'cd ".to_owned()
        + path.as_str()
        + " && cargo build && cargo run'";
    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Error starting_server");
    println!("Successfully started server");
    thread::sleep(Duration::from_secs(2));
    match start_node() {
        Ok(_) => {}
        Err(_) => {
            println!("Error starting node")
        }
    }
}

fn main() {
    let mut chain_running = false;
    let mut command = String::new();
    println!("Hello! Welcome to the blockchain interactive CLI !");
    println!("\n-------------------------------------------------\n\n\n");
    println!("There are several commands at your disposal : \n");
    loop {
        println!("Type <<start_node>> to start the blockchain\n");
        println!("Type <<create_account [id] [funds]>> to create a new account on the blockchain (note: only uint IDs are accepted)\n");
        println!("Type <<transfer [sender_id] [receiver_id] [funds]>> to transfer funds from one account to another\n");
        println!("Type <<balance [account_id]>> to read the balance of an existing account\n");
        command.clear();
        io::stdin().read_line(&mut command).unwrap();

        // Handling commands in main loop
        if command.contains("start_node") {
            if !chain_running {
                start_server();
                chain_running = true;
            } else {
                println!("Chain already running");
            }
        } else if command.contains("create_account") {
            match create_account(command.clone()) {
                Ok(_) => {}
                Err(_) => {
                    println!("Error creating account")
                }
            }
        } else if command.contains("transfer") {
            match transfer(command.clone()) {
                Ok(_) => {}
                Err(_) => {
                    println!("Error transfer funds")
                }
            }
        } else if command.contains("balance") {
            match read_balance(command.clone()) {
                Ok(_) => {}
                Err(_) => {
                    println!("Error reading balance");
                }
            }
        } else {
            println!("Unknown command {}, please try again", command);
        }
        println!("\n\n\n-------------------------------------------------\n\n\n");
    }
}

fn read_balance(cmd: String) -> Result<(), Box<dyn Error>> {
    let mut command: Vec<&str> = cmd.split(" ").collect();
    let id: u64;
    if command.len() == 3 && command[2] == "" {
        command.pop();
    }
    match command.len() {
        2 => match command.get(1).unwrap().to_string().trim().parse::<u64>() {
            Ok(i) => id = i,
            Err(_) => {
                println!("Error parsing account ID");
                return Err(Box::new(MyError("Oops".to_string())));
            }
        },
        _ => return Err(Box::new(MyError("Oops".to_string()))),
    }
    let resp = reqwest::blocking::get(BASE_URL.to_owned() + "/balance/" + id.to_string().as_str())?
        .text()?;
    println!("{:#?}", resp);
    Ok(())
}

fn transfer(cmd: String) -> Result<(), Box<dyn Error>> {
    let command: Vec<&str> = cmd.split(" ").collect();
    let sender: u64;
    let receiver: u64;
    let funds: u128;
    match command.len() {
        4 => {
            match command.get(1).unwrap().to_string().trim().parse::<u64>() {
                Ok(i) => sender = i,
                Err(_) => {
                    println!("Error parsing account ID");
                    return Err(Box::new(MyError("Oops".to_string())));
                }
            }
            match command.get(2).unwrap().to_string().trim().parse::<u64>() {
                Ok(i) => receiver = i,
                Err(_) => {
                    println!("Error parsing receiving account ID");
                    return Err(Box::new(MyError("Oops".to_string())));
                }
            }

            match command.get(3).unwrap().to_string().trim().parse::<u128>() {
                Ok(i) => funds = i,
                Err(_) => {
                    println!("Error parsing funds");
                    return Err(Box::new(MyError("Oops".to_string())));
                }
            }
        }
        _ => return Err(Box::new(MyError("Oops".to_string()))),
    }
    let resp = reqwest::blocking::get(
        BASE_URL.to_owned()
            + "/transfer/"
            + sender.to_string().as_str()
            + "/"
            + receiver.to_string().as_str()
            + "/"
            + funds.to_string().as_str(),
    )?
    .text()?;
    println!("{:#?}", resp);
    Ok(())
}

fn create_account(cmd: String) -> Result<(), Box<dyn Error>> {
    let command: Vec<&str> = cmd.split(" ").collect();
    let id: u64;
    let funds: u128;
    match command.len() {
        3 => {
            match command.get(1).unwrap().to_string().trim().parse::<u64>() {
                Ok(i) => id = i,
                Err(_) => {
                    println!("Error parsing account ID");
                    return Err(Box::new(MyError("Oops".to_string())));
                }
            }
            match command.get(2).unwrap().to_string().trim().parse::<u128>() {
                Ok(i) => funds = i,
                Err(_) => {
                    println!("Error parsing account initial funds");
                    return Err(Box::new(MyError("Oops".to_string())));
                }
            }
        }
        _ => return Err(Box::new(MyError("Oops".to_string()))),
    }
    let resp = reqwest::blocking::get(
        BASE_URL.to_owned()
            + "/create_account/"
            + id.to_string().as_str()
            + "/"
            + funds.to_string().as_str(),
    )?
    .text()?;
    println!("{:#?}", resp);
    Ok(())
}

fn start_node() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::blocking::get(BASE_URL.to_owned() + "/start_node")?.text()?;
    println!("{:#?}", resp);
    Ok(())
}
