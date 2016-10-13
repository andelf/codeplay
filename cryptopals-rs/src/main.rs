#![feature(inclusive_range_syntax)]

use std::str;


#[cfg(test)]
fn main() {
    println!("Hello Rust!");
}


#[cfg(not(test))]
mod challenges;

#[cfg(not(test))]
fn main() {

    challenges::set1_challenge3();

    challenges::set1_challenge4();
}
