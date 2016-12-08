extern crate rustc_demangle;

use std::env;

fn main() {
    let mut args = env::args();
    let name = args.nth(1).unwrap_or("".into());
    println!("got => {}", name);
    println!("demangle => {:?}", rustc_demangle::demangle(&name));
}

// demangle-rs '_ZN63_$LT$tikv..raftstore..store..store..Store$LT$T$C$$u20$C$GT$$GT$3run17hdfcfbe88fd87b678E'
