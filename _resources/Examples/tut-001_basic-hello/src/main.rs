// #![allow(unused)]

use std:: io;

fn main() {
    println!("what is your name?");
    let mut name = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin().read_line(&mut name)
        .expect("Didn't Receive User Input");
    println!("Hello {}! {}", name.trim_end(), greeting);
}
