#![allow(unused)] // silence warnings for unused variables
// import libraries
use std::io; // use std::io::* to import all things inside this scope
use rand::Rng;
// to import specific attributes
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // If else statements
    let age = 8;
    if (age >= 1) && (age <= 18){
        println!("Important birthday");
    } else if (age == 21 || age == 50){
        println!("Important birthday");
    } else if (age >= 65){
        println!("Important birthday");
    } else {
        println!("Not an important birthday")
    }

    // Ternary operator
    let mut my_age = 37;
    let can_vote = if my_age >= 18{
        true
    } else {
        false
    };
    println!("Can vote: {}", can_vote);

    // Match statements
    let age2 = 18;
    match age2{
        1..=18 => println!("Important Birthday"),
        21 | 50 => print!("Important Birthday"),
        65..=i32::MAX => println!("Important Birthday"),
        _ => println!("Not an important birthday"),
    };

    my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age){
        Ordering::Less => println!("Cant Vote"),
        Ordering::Equal => println!("Can Vote"),
        Ordering::Greater => println!("Can Vote"),
    }
}