#![allow(unused)] // silence warnings for unused variables
// import libraries
use std::io;
use std::ptr::null; // use std::io::* to import all things inside this scope
use rand::Rng;
// to import specific attributes
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let mut heroes: HashMap<&str, &str> = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Spiderman", "Peter Parker");
    heroes.insert("Ironman", "Tony Stark");

    for (k, v) in heroes.iter(){
        println!("{} = {}", k, v);
    }
    println!("Length: {}", heroes.len());
    if heroes.contains_key("Spiderman"){
        let spiderman = heroes.get("Spiderman");
        match spiderman{
            Some(x) => println!("Spiderman is {}", x),
            None => println!("Spidermen is not here"),
        }
    }

}