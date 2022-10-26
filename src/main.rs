#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn get_sum_3(x: i32, y: i32) -> i32 {
    return x + y;
}

fn main() {
    println!("{}", get_sum_3(5, 4));
}
