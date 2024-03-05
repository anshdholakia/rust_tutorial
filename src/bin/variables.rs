#![allow(unused)] // silence warnings for unused variables
// import libraries
use std::io; // use std::io::* to import all things inside this scope
use rand::Rng;
// to import specific attributes
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // Rust is statically typed so all types must be defined
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    // shadowing is wher you define two variables with same names but diff datatypes
    let age = "47";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned as a number");
    age = age + 1;
    println!("I'm {}, and I want {}", age, ONE_MIL);
        
}
