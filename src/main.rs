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
    let str2 = str1;
    println!("Hello {}", str1);
/* THIS CODE WON'T COMPILE:
   Compiling rust_tutorial v0.1.0 (/Users/datique/lab/training/rust/derek-banas-video-course/rust_tutorial)
error[E0382]: borrow of moved value: `str1`
  --> src/main.rs:24:26
   |
22 |     let str1 = String::from("World");
   |         ---- move occurs because `str1` has type `String`, which does not implement the `Copy` trait
23 |     let str2 = str1;
   |                ---- value moved here
24 |     println!("Hello {}", str1);
   |                          ^^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0382`.
error: could not compile `rust_tutorial` due to previous error

 *  The terminal process "cargo 'run', '--package', 'rust_tutorial', '--bin', 'rust_tutorial'" terminated with exit code: 101. 
 *  Terminal will be reused by tasks, press any key to close it. 
*/
}
