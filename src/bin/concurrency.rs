#![allow(unused)] // silence warnings for unused variables
// import libraries
use std::io; // use std::io::* to import all things inside this scope
use rand::{thread_rng, Rng};
// to import specific attributes
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::thread;
use std::time::Duration;

fn main() {
    // create a thread
    let thread1 = thread::spawn( || {
        for i in 1..25{
            println!("Spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..20{
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    thread1.join().unwrap();
}