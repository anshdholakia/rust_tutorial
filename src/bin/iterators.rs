#![allow(unused)] // silence warnings for unused variables
// import libraries
use std::io; // use std::io::* to import all things inside this scope
use rand::Rng;
// to import specific attributes
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let mut arr = [1, 2, 3, 4];
    // it borrows values from the array. you cannot change any values from the array using this iterator
    for val in arr.iter(){
        println!("{}", val);
    }
    let mut iter1 = arr.iter();
    println!("1st : {:?}", iter1.next());
}