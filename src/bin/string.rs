#![allow(unused)] // silence warnings for unused variables
// import libraries
use std::{char, io}; // use std::io::* to import all things inside this scope
use rand::Rng;
// to import specific attributes
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");
    for word in st1.split_whitespace(){
        println!("{}", word);
    }
    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    let st3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    // sort characters
    v1.sort();
    // remove duplicates
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }
    let st4 = "Random String";
    let mut st5 = st4.to_string();
    println!("{}", st5);
    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("String length: {}", st6.len());
    st5.clear();

    // combine strings
    let mut st6 = String::from("Just some");
    let st7 = String::from(" words");
    let st8 = st6 + &st7;
    // println!("{}", st6); // this line wont work since st6 is now in st8
    for char in st8.bytes() {
        println!("{}", char);
        
    }
    
    
    
    
}