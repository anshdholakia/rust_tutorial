#![allow(unused)] // silence warnings for unused variables
// import libraries
use std::io; // use std::io::* to import all things inside this scope
use rand::Rng;
// to import specific attributes
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // Instead of putting a silencer for unused variables at the top of your file, if you want to ignore a specific unused variables, put an underscore before the variable name

    // Unsigned integer: u8, u16, u32, u64, u128, usize
    // Signed integer: i8, i16, i32, i64, i128, isize
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);

    // boolean
    let is_true = true;

    // character
    let character = 'A';
}