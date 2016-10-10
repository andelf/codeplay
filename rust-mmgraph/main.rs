#![feature(box_syntax)]

use std::io;
use std::io::prelude::*;
use std::fs::File;

#[path = "lib.rs"]
mod mmgraph;
use mmgraph::Graph;


fn run_assignment() -> io::Result<()> {
    let mut s = String::new();
    let mut f = try!(File::open("edges.txt"));
    try!(f.read_to_string(&mut s));

    let header: Vec<usize> = s.splitn(3, char::is_whitespace)
                              .take(2)
                              .map(|n| n.parse().unwrap())
                              .collect();
    let number_of_nodes = header[0];
    let _numer_of_edges = header[1];

    let mut g = Graph::with_capacity(number_of_nodes);

    s.lines()
     .skip(1)
     .map(|line| {
         let vals: Vec<i64> = line.trim()
                                  .split(' ')
                                  .map(|s| s.parse::<i64>().unwrap())
                                  .collect();
         let u = vals[0];
         let v = g.find_or_create_node(vals[1]);
         let w = vals[2];

         g.with_found_or_new_node(u).create_rel_to(v, w);
     })
     .last();

    println!("\n{}",
             g.to_dot());
    Ok(())

}

fn main() {
    let mut g = Graph::<_, i32>::new();
    let v = g.create_node("A");
    let x = g.create_node("B");
    let u = g.create_node("C");
    let w = g.create_node("D");
    let q = g.create_node("E");

    // println!("\n{}", g.to_dot());
    g.node_mut(v).as_mut().map(|v| v.create_rel_to(x, 233));
    g.with_new_node("F").create_rel_to(v, 45);
    g.node_mut(v).as_mut().map(|v| v.create_rel_to(u, 233));
    g.node_mut(q).as_mut().map(|q| q.create_rel_to(v, 233));

    // println!("\n{}", g.to_dot());


    run_assignment();
    // assert!(!q.has_rel());
    //
    // v.create_rel_to(&q);
    // assert!(q.has_rel());
    //
    // v.create_rel_to(&w);
    // v.create_rel_to(&x);
    // x.create_rel_to(&v);
    // u.create_rel_to(&x);
    // println!("g: {:?}", g);
    //
    // println!("V => {:?}", *v);
    // assert!(v.has_rel());
    //
    //
}
