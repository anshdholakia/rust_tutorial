#![allow(unused)] // silence warnings for unused variables
// import libraries
use std::io; // use std::io::* to import all things inside this scope
use rand::Rng;
// to import specific attributes
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// Result has two variants Ok and Err
// enum Result <T, E> {
// Ok(T),
// Err(E), }
// Where T represents the data typeof the value returns and E
// the type of error

fn main() {
    // panic!("Terrible Error!"); // To print error
    // let arr = [1, 2];
    // println!("{}", arr[10]); // get an error
    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {panic!("problem creating file: {:?}", error);},
    };
    write!(output, "Just some\nRandom words").expect("Failed to write to the file");

    let input = File::open(path).unwrap();
    let buffered= BufReader::new(input);
    for line in buffered.lines(){
        println!("{}", line.unwrap());
    }

    let output2 = File::create("rand.txt");
    let output2= match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt"){
                Ok(fc) => fc,
                Err(error) => panic!("Cant create file: {:?}", error),
            },
            _ => panic!("Error opening file: {:?}", error),
        },
    };
}
