#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn print_str(x: String) {
    println!("A string {}", x);
}

fn print_return_str(x: String) -> String {
    println!("A string {}", x);
    x
}

fn change_string(name: &mut String) {
    name.push_str(" is happy");
    println!("Message : {}", name);
}

fn main() {
    let mut str1 = String::from("Derek");
    let str2 = str1.clone();
    
    //print_str(str1);

    //let str3 = print_return_str(str1);
    //println!("str3 = {}", str3);

    change_string(&mut str1);
}
