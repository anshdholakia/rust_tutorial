#![allow(unused)] // silence warnings for unused variables
// import libraries
use std::io; // use std::io::* to import all things inside this scope
use rand::Rng;
// to import specific attributes
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // Functions without a name (basically lambda functions)
    // let var_name = | parameters | { body };
    let can_vote = |age: i32| {
        age >= 18
    };
    println!("Can I vote? : {}", can_vote(18));
    let mut sampl = 5;
    let print_var= || println!("sampl: {}", sampl);
    print_var();
    // The compiler sees that sampl is being mutably borrowed by change_var, which prevents any further use of sampl until change_var is dropped or called, as mutable borrows are exclusive.
    let mut change_var= || sampl += 1;
    change_var();
    println!("sampl: {}", sampl);
    sampl = 10;
    println!("sampl: {}", sampl);

    fn use_func<T>(a: i32, b: i32, func: T) -> i32 
    where T: Fn(i32, i32) -> i32 { // define the generic type
        func(a, b)
    }
    let sum = |a, b| a+b;
    let prod = |a, b| a*b;
    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("5 * 4 = {}", use_func(5, 4, prod));
}