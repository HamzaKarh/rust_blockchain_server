use std::process::Command;
use std::{io, thread};


fn main() {

    thread::spawn(|| {Command::new("sh")
        .arg("-c")
        .arg("gnome-terminal -- bash -c 'cd blockchain && cargo build && cargo run'")
        .output()
        .expect("error");});

    Command::new("sh")
        .arg("-c")
        .arg("gnome-terminal -- bash -c 'cd console && cargo build && cargo run'")
        .output()
        .expect("error");


}
