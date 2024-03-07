#![allow(unused)] // silence warnings for unused variables
// import libraries
use std::io; // use std::io::* to import all things inside this scope
use rand::Rng;
// to import specific attributes
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // box is a smart pointer basically stores data on the heap instead of the stack
    let b_int1 = Box::new(10);
    println!("b_int1 = {}", b_int1);
    
    // creating a binary tree ds using box
    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>, // this makes sure that left can hold treenode and a null pointer
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }
    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode {left: None, right: None, key: key}
        }
        pub fn left(mut self, node: TreeNode<T>) -> Self{
            self.left = Some(Box::new(node));
            return self;
        }
        pub fn right(mut self, node: TreeNode<T>) -> Self{
            self.right = Some(Box::new(node));
            return self;
        }
    }

    let root = TreeNode::new(10).left(TreeNode::new(1)).right(TreeNode::new(11));


}