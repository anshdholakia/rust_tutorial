#![allow(unused)] use core::num;
// silence warnings for unused variables
// import libraries
use std::io; // use std::io::* to import all things inside this scope
use rand::Rng;
// to import specific attributes
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn say_hello(){
    println!("Hello");
}

fn get_sum(x: i32, y: i32){
    println!("{} + {} = {}", x, y, x+y);
}

fn get_sum_2(x: i32, y: i32) -> i32{
    return x+y;
}

fn get_sum_3(x: i32, y: i32) -> i32{
    x + y
}

fn return_plus_and_mult(x: i32, y: i32) -> (i32, i32){
    return (x+y, x*y);
}

fn sum_list(listt: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in listt {
        sum+=i;
    }
    return sum;
}

fn main() {
    // get_sum(5, 4);
    println!("{}", get_sum_2(3, 4));
    let (val1, val2) = return_plus_and_mult(3, 4);
    println!("{}, {}", val1, val2);
    let num_list = vec![1, 2, 3, 4, 5];
    println!("Sum of list: {}", sum_list(&num_list));
    
    
}