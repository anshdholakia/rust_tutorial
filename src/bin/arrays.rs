#![allow(unused)] // silence warnings for unused variables
// import libraries
use std::io; // use std::io::* to import all things inside this scope
use rand::Rng;
// to import specific attributes
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // Arrays must of of the same datatype
    let arr1 = [1, 2, 3, 4];
    println!("1st: {}", arr1[0]);
    println!("Length: {}", arr1.len());
    
}