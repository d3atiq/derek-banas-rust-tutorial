#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn get_sum_2(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    println!("{}", get_sum_2(5, 4));
}
