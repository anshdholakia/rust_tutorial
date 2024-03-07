#![allow(unused)] // silence warnings for unused variables
// import libraries
use std::io; // use std::io::* to import all things inside this scope
use rand::Rng;
// to import specific attributes
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// Declare that we wnat to user restaurant module
#[path = "../restaurant/mod.rs"] // dont need this if in main.rs
mod restaurant;
use crate::restaurant::order_food;

fn main() {
    // Crates: Modules that produce a library or executable
    // Modules: Organize and handle privacy
    // Packages: Build, test, and share crates
    // Paths: A way of naming an item such as a struct, function
    order_food();
}