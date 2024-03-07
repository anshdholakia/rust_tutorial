#![allow(unused)]
// silence warnings for unused variables
// import libraries
use std::io; // use std::io::* to import all things inside this scope
use rand::Rng;
// to import specific attributes
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::f32::consts::*;
use std::cmp::Ordering;

fn main() {
    struct Customer{
        name: String,
        address: String,
        balance: f32,
    }
    let mut bob = Customer{
        name: String::from("Bob"),
        address: String::from("3200 Chestnut St"),
        balance: 234.50
    };
    bob.address = String::from("505 Main St");

    struct Rectangle<T, U>{
        length: T,
        width: U,
    };
    let rec: Rectangle<i32, f32> = Rectangle{length: 23, width: 5.0};

    // trait -> tie struct properties to traits
    trait Shape {
        // define the functions
        fn new(length: f32, width: f32) -> Self; // constructor
        fn area(&self) -> f32;
    };

    struct Parallelogram {length: f32, width: f32};
    struct Circle {length: f32, width: f32};

    // implement parallelogram trait
    impl Shape for Parallelogram{
        fn new(length: f32, width: f32) -> Parallelogram {
            return Parallelogram{length, width};
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    };

    // implement circle trait
    impl Shape for Circle{
        fn new(length: f32, width: f32) -> Circle {
            return Circle{length, width};
        }
        fn area(&self) -> f32 {
            return PI*self.length*self.width;
        }
    };

    // define objects
    let para: Parallelogram = Shape::new(10.0, 10.0);
    let circle: Circle = Shape::new(34.0, 34.0);
    println!("Parallelogram to area = {}", para.area());
    println!("Circle area = {}", circle.area());
    
}