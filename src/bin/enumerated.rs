#![allow(unused)] // silence warnings for unused variables
// import libraries
use std::io; // use std::io::* to import all things inside this scope
use rand::Rng;
// to import specific attributes
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // custom data types
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl Day {
        fn is_weekend(&self) -> bool{
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false
            }
        }
    }

    let today = Day::Sunday;
    match today{
        Day::Monday => println!("Boring lol"),
        _ => println!("Ok day")
    };

    println!("Is today a weekend?: {}", today.is_weekend());
    
}