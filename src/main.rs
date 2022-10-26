#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// Stack : Stores values in a last in first out format
// Data on the stack must have a defined fixed size

// Heap : When putting data on the heap you request a 
// certain amount of space. The OS finds space available and 
// returns an address for that space called a pointer.

// RULES
    // 1. Each value has a variable that's called its owner
    // 2. There is only one owner at a time
    // 3. When the owner goes out of scope the value disappears

fn main() {
    let str1 = String::from("World");
    let str2 = str1.clone();
    println!("Hello {}", str1);
}
