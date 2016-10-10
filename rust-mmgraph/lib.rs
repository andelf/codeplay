#![feature(box_syntax)]

use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;

mod rawlink;
use self::rawlink::Rawlink;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Outgoing,
    Incoming,
    Both,
}

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Option<Box<Node>>>,
    rels: Vec<Option<Box<Relationship>>>,
    reuse_nodes: Vec<usize>,
    reuse_rels: Vec<usize>,
}

#[derive(Debug)]
pub struct Node {
    id: usize,
    name: i32,
    next_start_rel: Option<usize>,
    next_end_rel: Option<usize>, /* TODO: support prop
                                  * prop: BTreeMap<String, Property>,
                                  * TODO: label */
}

pub struct NodeMut<'a> {
    node: Rawlink<Node>,
    graph: Rawlink<Graph>,
    _marker: PhantomData<&'a u8>
}

impl<'a> NodeMut<'a> {
    pub fn create_rel_to(&mut self, other: &Node) -> RelationshipMut {
        let n = unsafe { self.node.resolve().expect("can't reuse after delete.") };
        if let Some(g) = unsafe { self.graph.resolve_mut() } {
            let id = if let Some(reuse_id) = g.reuse_rels.pop() {
                reuse_id
            } else {
                g.rels.push(None);
                g.rels.len() - 1
            };

            let mut rel = Relationship {
                id: id,
                start_node: n.id,
                start_prev: None,
                start_next: None,
                end_node: other.id,
                end_prev: None,
                end_next: None,
            };
            // handle relationship links
            if let Some(next_rid) = n.next_start_rel {
                rel.start_next = Some(next_rid);
                g.rels[next_rid].as_mut().map(|r| r.start_prev = Some(id));
            }
            // FIXME: unsafe change n.next_rel
            g.nodes[n.id].as_mut().map(|n| n.next_start_rel = Some(id));

            if let Some(next_rid) = other.next_end_rel {
                rel.end_next = Some(next_rid);
                g.rels[next_rid].as_mut().map(|r| r.end_prev = Some(id));
            }
            g.nodes[other.id].as_mut().map(|n| n.next_end_rel = Some(id));

            g.rels[id] = Some(box rel);
            RelationshipMut {
                rel: Rawlink::some(g.rels[id].as_mut().unwrap()),
                graph: self.graph,
            }
        } else {
            panic!("mem error!")
        }
    }

    pub fn delete(&mut self) {
        unimplemented!()
    }

    pub fn iter_rels(&self) -> ::std::slice::Iter<Relationship> {
        unimplemented!()
    }

    pub fn iter_rels_of_dir(&self, dir: Direction) -> ::std::slice::Iter<Relationship> {
        unimplemented!()
    }

    pub fn get_degree(&self) -> usize {
        self.get_degree_of_dir(Direction::Both)
    }

    pub fn get_degree_of_dir(&self, dir: Direction) -> usize {
        let n = unsafe { self.node.resolve().expect("can't reuse after delete.") };
        let g = unsafe { self.graph.resolve().expect("mem error") };

        let mut degree = 0;

        match dir {
            Direction::Outgoing => {
                let mut next = n.next_start_rel;
                while let Some(mut rid) = next {
                    next = g.rels[rid].as_ref().unwrap().start_next;
                    degree += 1;
                }
            }
            Direction::Incoming => {
                let mut next = n.next_end_rel;
                while let Some(mut rid) = next {
                    next = g.rels[rid].as_ref().unwrap().end_next;
                    degree += 1;
                }
            }
            _ => {
                return self.get_degree_of_dir(Direction::Outgoing) +
                       self.get_degree_of_dir(Direction::Incoming)
            }
        }
        degree
    }
}

impl<'a> Deref for NodeMut<'a> {
    type Target = Node;
    fn deref(&self) -> &Node {
        unsafe { self.node.resolve().expect("Can't reuse a deleted node.") }
    }
}

impl<'a> DerefMut for NodeMut<'a> {
    fn deref_mut(&mut self) -> &mut Node {
        unsafe { self.node.resolve_mut().expect("Can't reuse a deleted node.") }
    }
}


#[derive(Debug)]
pub struct Relationship {
    id: usize, // idx in Graph
    start_node: usize,
    start_prev: Option<usize>,
    start_next: Option<usize>,
    end_node: usize,
    end_prev: Option<usize>,
    end_next: Option<usize>, /* TODO: relationship type, property
                              * prop: BTreeMap<String, Property>, */
}

pub struct RelationshipMut {
    rel: Rawlink<Relationship>,
    graph: Rawlink<Graph>,
}

impl RelationshipMut {
    pub fn start_node(&self) -> &Node {
        unimplemented!()
    }

    pub fn end_node(&self) -> &Node {
        unimplemented!()
    }

    pub fn other_node(&self, node: &Node) -> &Node {
        unimplemented!()
    }

    pub fn nodes(&self) -> ::std::slice::Iter<Node> {
        unimplemented!()
    }
}


impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: vec![],
            rels: vec![],
            reuse_nodes: vec![],
            reuse_rels: vec![],
        }
    }

    pub fn with_capacity(n: usize) -> Graph {
        Graph {
            nodes: Vec::with_capacity(n),
            rels: Vec::with_capacity(n),
            reuse_nodes: vec![],
            reuse_rels: vec![],
        }
    }

    pub fn create_node<'a, 'b: 'a>(&'a mut self) -> NodeMut<'b> {
        self.create_node_with_name(2333)
    }

    pub fn create_node_with_name<'a, 'b: 'a>(&'a mut self, name: i32) -> NodeMut<'b> {
        let id = if let Some(reuse_id) = self.reuse_nodes.pop() {
            reuse_id
        } else {
            self.nodes.push(None);
            self.nodes.len() - 1
        };
        let node = Node {
            id: id,
            name: name,
            next_start_rel: None,
            next_end_rel: None,
        };
        self.nodes[id] = Some(box node);

        NodeMut {
            node: Rawlink::some(self.nodes[id].as_mut().unwrap()),
            graph: Rawlink::some(self),
            _marker: PhantomData
        }
    }

    pub fn get_node_by_id(&self, id: usize) -> Option<&Node> {
        self.nodes.get(id).and_then(|n| n.as_ref().map(|n| &**n))
    }

    pub fn get_rel_by_id(&self, id: usize) -> Option<&Relationship> {
        self.rels.get(id).and_then(|r| r.as_ref().map(|r| &**r))
    }

    pub fn iter_all_nodes(&self) -> Vec<&Node> {
        self.nodes
            .iter()
            .filter(|n| n.is_some())
            .map(|n| n.as_ref().map(|n| &**n).unwrap())
            .collect::<Vec<&Node>>()
    }

    pub fn iter_all_rels(&self) -> Vec<&Relationship> {
        self.rels
            .iter()
            .filter(|r| r.is_some())
            .map(|r| r.as_ref().map(|r| &**r).unwrap())
            .collect::<Vec<&Relationship>>()
    }

    pub fn dump(&self) {
        println!("");
        self.nodes
            .iter()
            .filter(|n| n.is_some())
            .map(|n| {
                println!("{:?}", n.as_ref().unwrap());
            })
            .last();

        self.rels
            .iter()
            .filter(|r| r.is_some())
            .map(|r| {
                println!("{:?}", r.as_ref().unwrap());
            })
            .last();
    }

    pub fn to_dot(&self) -> String {
        let mut dot = String::new();

        dot.push_str("digraph G {\n");
        for n in &self.nodes {
            if let Some(n) = n.as_ref() {
                dot.push_str(&format!("    {} [label = \"{}:{}\"];\n", n.id, n.id, n.name));
            }
        }

        self.rels
            .iter()
            .filter(|r| r.is_some())
            .map(|r| {
                let r = r.as_ref().unwrap();
                dot.push_str(&format!("    {} -> {} [label = \"{}\"];\n",
                                      r.start_node,
                                      r.end_node,
                                      r.debug()));
            })
            .last();
        dot.push_str("}\n");
        dot
    }
}


impl Node {
    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn has_rel(&self) -> bool {
        self.next_start_rel.or(self.next_end_rel).is_some()
    }
}


impl Relationship {
    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn debug(&self) -> String {
        format!("[id={} {}->{} {:?}/{:?} {:?}/{:?}]",
                self.id,
                self.start_node,
                self.end_node,
                self.start_prev,
                self.start_next,
                self.end_prev,
                self.end_next)
    }
}


#[test]
fn test_graph_create_node() {
    let mut g = Graph::new();
    let mut v = g.create_node();
    let mut x = g.create_node_with_name(666);
    let mut u = g.create_node_with_name(999);
    let w = g.create_node_with_name(999);
    let q = g.create_node_with_name(555);

    assert!(!q.has_rel());

    v.create_rel_to(&u);
    v.create_rel_to(&w);
    v.create_rel_to(&x);
    x.create_rel_to(&v);
    u.create_rel_to(&x);
    println!("g: {:?}", g);

    assert!(v.has_rel());
    g.dump();

    println!("\n {}", g.to_dot());

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
