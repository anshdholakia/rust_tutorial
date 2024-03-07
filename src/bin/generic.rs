#![allow(unused)] // silence warnings for unused variables
// import libraries
use std::io; // use std::io::* to import all things inside this scope
use rand::Rng;
// to import specific attributes
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;
use std::ops::Sub;

// We would want to use generic when we want to use functions which can work with multiple different types of data
fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T{
    return x + y;
}

fn get_sub_gen<T>(x: T, y: T) -> T where T: Sub<Output = T>{
    return x - y;
}

fn main() {
    print!("1 + 2 = {}", get_sum_gen(1, 2));   
}