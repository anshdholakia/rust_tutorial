#![allow(unused)] // silence warnings for unused variables
// import libraries
use std::io; // use std::io::* to import all things inside this scope
use rand::Rng;
// to import specific attributes
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::thread;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

fn main() {
    pub struct Bank{
        balance: f32,
    }

    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32){
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < 5.00 {
            println!("Current balance: {}", bank_ref.balance);
        } else {
            bank_ref.balance -= amt;
            println!("Customer withdrew: {}. Current balance: {}", amt, bank_ref.balance);
        }
    }

    fn customer(the_bank: Arc<Mutex<Bank>>){
        withdraw(&the_bank, 10.00);
    }

    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank{balance: 100.0}));
    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(||{
            customer(bank_ref);
        })
    });

    for handle in handles{
        handle.join().unwrap();
    }
    println!("Total {}", bank.lock().unwrap().balance);

    /* 
    This code will not work because Rust's safety guarantees prevent data races by ensuring that a piece of data can either be mutably borrowed by a single reference or immutably borrowed by multiple references, but not both at the same time.
    fn withdraw(the_bank: &mut Bank, amt: f32){
        the_bank.balance -= amt;
    }
    let mut bank = Bank{balance: 100.0};
    withdraw(&mut bank, 30.0);
    println!("Balance remaining in bank: {}", bank.balance);

    fn customer(the_bank: &mut Bank){
        withdraw(the_bank, 30.0);
    }

    thread::spawn(|| {
        customer(&mut bank)
    }).join().unwrap();
    */
}