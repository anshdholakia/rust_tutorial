#![allow(unused)] // silence warnings for unused variables
// import libraries
use std::io; // use std::io::* to import all things inside this scope
use rand::Rng;
// to import specific attributes
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// RULES IN OWNERSHIP:
// 1. Each value has a variable called its owner
// 2. There is only one owner at a time
// 3. When the owner goes out of the scope the value disappears

fn print_str(string: String){
    println!("{}", string);
}

// because its parameter is of type String, not &String or &mut String. Once a value has been passed to a function like this, the function owns it, and the original variable (st1 in this case) no longer has access to it.
fn return_n_print(string: String) -> String{
    println!("{}", string);
    return string.clone();
}

fn change_str(string: &mut String){
    string.push_str(" is happy");
    println!("{}", string);
}

fn main() {
    let st1 = String::from("World");
    // let st2 = st1; // st1 doesnt exist anymore
    let st2 = st1.clone();
    let mut st3 = return_n_print(st1);
    change_str(&mut st3);
    println!("{}", st3);
}