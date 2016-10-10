#![feature(box_syntax)]

#[path = "lib.rs"]
mod mmgraph;

use mmgraph::Graph;



fn main() {
    let mut g = Graph::new();
    let mut v = g.create_node();
    let mut x = g.create_node_with_name(666);
    let mut u = g.create_node_with_name(999);
    let w = g.create_node_with_name(999);
    let q = g.create_node_with_name(555);

    assert!(!q.has_rel());

    v.create_rel_to(&q);
    assert!(q.has_rel());

    v.create_rel_to(&w);
    v.create_rel_to(&x);
    x.create_rel_to(&v);
    u.create_rel_to(&x);
    println!("g: {:?}", g);

    println!("V => {:?}", *v);
    assert!(v.has_rel());

    println!("\n{}", g.to_dot());
}
