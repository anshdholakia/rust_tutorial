#![allow(unused)] // silence warnings for unused variables
// import libraries
use std::io; // use std::io::* to import all things inside this scope
use rand::Rng;
// to import specific attributes
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let arr = [1, 2, 24, 35, 45, 32, 53, 45, 2, 34];
    let mut loop_idx = 0;
    loop {
        if loop_idx>=10{
            break;
        }
        println!("{}", arr[loop_idx]);
        loop_idx+=1;
    }

    loop_idx = 0;
    // while loop
    while loop_idx < arr.len(){
        println!("Arr: {}", arr[loop_idx]);
        loop_idx+=1;
    }

    // for loop
    loop_idx = 0;
    for val in arr.iter() {
        println!("Val: {}", val);
        
    }
}