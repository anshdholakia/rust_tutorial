#![allow(unused)] // silence warnings for unused variables

// import libraries
use std::io; // use std::io::* to import all things inside this scope
use rand::Rng;
// to import specific attributes
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // things with ! are called a macro
    println!("What is your name?");
    // letting the variable be mut meaning mutable or you can leave it to let (default immutable)
    let mut name = String::new();
    let greeting = "Nice to meet you";
    io::stdin().read_line(&mut name)
        .expect("Didnt receive input");
    // name.trim_end is necessary since a newline is included in name
    println!("Hello {}!, {}", name.trim_end(), greeting);
}