#![feature(box_syntax)]

use std::ops::{Deref, DerefMut};
use std::fmt;

mod rawlink;
use self::rawlink::Rawlink;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Outgoing,
    Incoming,
    Both,
}


#[derive(Debug)]
pub struct Node<N> {
    id: usize,
    name: N,
    next_start_rel: Option<usize>,
    next_end_rel: Option<usize>, /* TODO: support prop
                                  * prop: BTreeMap<String, Property>,
                                  * TODO: label */
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NodeIx(usize);

pub struct NodeMut<'a, N: 'a, E: 'a> {
    node: NodeIx,
    graph: &'a mut Graph<N, E>,
}

impl<'a, N, E> Deref for NodeMut<'a, N, E> {
    type Target = Node<N>;
    fn deref(&self) -> &Self::Target {
        &**self.graph.nodes[self.node.0].as_ref().unwrap()
    }
}

impl<'a, N, E> DerefMut for NodeMut<'a, N, E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut **self.graph.nodes[self.node.0].as_mut().unwrap()
    }
}

impl<'a, N, E> NodeMut<'a, N, E> {
    pub fn create_rel_to(&mut self, other: NodeIx, weight: E) -> RelationshipIx {
        let &Node { id: my_id, next_start_rel: my_next_start_rel_id, .. } =
            &**self.graph.nodes[self.node.0].as_ref().unwrap();
        let id = if let Some(reuse_id) = self.graph.reuse_rels.pop() {
            reuse_id
        } else {
            self.graph.rels.push(None);
            self.graph.rels.len() - 1
        };

        let &Node { id: other_id, next_end_rel: other_next_end_rel_id, .. } =
            &**self.graph.nodes[other.0].as_ref().unwrap();

        let mut rel = Relationship {
            id: id,
            weight: weight,
            start_node: my_id,
            start_prev: None,
            start_next: None,
            end_node: other_id,
            end_prev: None,
            end_next: None,
        };
        // handle relationship links
        if let Some(next_rid) = my_next_start_rel_id {
            rel.start_next = Some(next_rid);
            self.graph.rels[next_rid].as_mut().map(|r| r.start_prev = Some(id));
        }
        // FIXME: unsafe change n.next_rel
        self.graph.nodes[my_id].as_mut().map(|n| n.next_start_rel = Some(id));

        if let Some(next_rid) = other_next_end_rel_id {
            rel.end_next = Some(next_rid);
            self.graph.rels[next_rid].as_mut().map(|r| r.end_prev = Some(id));
        }
        self.graph.nodes[other_id].as_mut().map(|n| n.next_end_rel = Some(id));

        self.graph.rels[id] = Some(box rel);
        RelationshipIx(id)
    }

    pub fn delete(&mut self) {
        unimplemented!()
    }

    pub fn iter_rels(&self) -> ::std::slice::Iter<Relationship<E>> {
        unimplemented!()
    }

    pub fn iter_rels_of_dir(&self, dir: Direction) -> ::std::slice::Iter<Relationship<E>> {
        unimplemented!()
    }

    pub fn get_degree(&self) -> usize {
        self.get_degree_of_dir(Direction::Both)
    }

    pub fn get_degree_of_dir(&self, dir: Direction) -> usize {
        let n = self.graph.nodes[self.node.0].as_ref().unwrap();

        let mut degree = 0;

        match dir {
            Direction::Outgoing => {
                let mut next = n.next_start_rel;
                while let Some(mut rid) = next {
                    next = self.graph.rels[rid].as_ref().unwrap().start_next;
                    degree += 1;
                }
            }
            Direction::Incoming => {
                let mut next = n.next_end_rel;
                while let Some(mut rid) = next {
                    next = self.graph.rels[rid].as_ref().unwrap().end_next;
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


#[derive(Debug)]
pub struct Relationship<E> {
    id: usize, // idx in Graph
    weight: E,
    start_node: usize,
    start_prev: Option<usize>,
    start_next: Option<usize>,
    end_node: usize,
    end_prev: Option<usize>,
    end_next: Option<usize>, /* TODO: relationship type, property
                              * prop: BTreeMap<String, Property>, */
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RelationshipIx(usize);

pub struct RelationshipMut<'a, N: 'a, E: 'a> {
    rel: RelationshipIx,
    graph: &'a mut Graph<N, E>,
}

impl<'a, N, E> Deref for RelationshipMut<'a, N, E> {
    type Target = Relationship<E>;
    fn deref(&self) -> &Self::Target {
        &**self.graph.rels[self.rel.0].as_ref().unwrap()
    }
}

impl<'a, N, E> DerefMut for RelationshipMut<'a, N, E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut **self.graph.rels[self.rel.0].as_mut().unwrap()
    }
}

impl<'a, N, E> RelationshipMut<'a, N, E> {
    pub fn start_node(&self) -> NodeIx {
        NodeIx(self.graph.rels[self.rel.0].as_ref().unwrap().start_node)
    }

    pub fn end_node(&self) -> NodeIx {
        NodeIx(self.graph.rels[self.rel.0].as_ref().unwrap().end_node)
    }

    pub fn other_node(&self, node: NodeIx) -> NodeIx {
        if self.start_node() == node {
            self.end_node()
        } else if self.end_node() == node {
            self.start_node()
        } else {
            panic!("node is not belong to this rel")
        }
    }

    pub fn nodes(&self) -> ::std::slice::Iter<Node<N>> {
        unimplemented!()
    }
}


#[derive(Debug)]
pub struct Graph<N, E> {
    nodes: Vec<Option<Box<Node<N>>>>,
    rels: Vec<Option<Box<Relationship<E>>>>,
    reuse_nodes: Vec<usize>,
    reuse_rels: Vec<usize>,
}

impl<N: fmt::Debug + PartialEq, E> Graph<N, E> {
    pub fn new() -> Graph<N, E> {
        Graph {
            nodes: vec![],
            rels: vec![],
            reuse_nodes: vec![],
            reuse_rels: vec![],
        }
    }

    pub fn with_capacity(n: usize) -> Graph<N, E> {
        Graph {
            nodes: Vec::with_capacity(n),
            rels: Vec::with_capacity(n),
            reuse_nodes: vec![],
            reuse_rels: vec![],
        }
    }

    pub fn find_or_create_node(&mut self, name: N) -> NodeIx {
        let nix = self.nodes
                      .iter()
                      .find(|n| n.as_ref().map_or(false, |n| n.name == name))
                      .and_then(|n| n.as_ref())
                      .map(|n| NodeIx(n.id));
        if let Some(nix) = nix {
            nix
        } else {
            self.create_node(name)
        }
    }

    pub fn create_node(&mut self, name: N) -> NodeIx {
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

        NodeIx(id)
    }

    pub fn node(&self, id: NodeIx) -> Option<&Node<N>> {
        self.nodes.get(id.0).and_then(|n| n.as_ref().map(|n| &**n))
    }

    pub fn node_mut(&mut self, id: NodeIx) -> Option<NodeMut<N, E>> {
        Some(NodeMut {
            node: id,
            graph: self,
        })
    }

    pub fn with_new_node(&mut self, name: N) -> NodeMut<N, E> {
        let id = self.create_node(name);
        NodeMut {
            node: id,
            graph: self,
        }
    }

    pub fn with_found_or_new_node(&mut self, name: N) -> NodeMut<N, E> {
        let node = self.nodes
                       .iter()
                       .find(|n| n.as_ref().map_or(false, |n| n.name == name))
                       .and_then(|n| n.as_ref())
                       .map(|n| NodeIx(n.id));
        if let Some(id) = node {
            NodeMut {
                node: id,
                graph: self,
            }
        } else {
            self.with_new_node(name)
        }
    }

    pub fn rel(&self, id: RelationshipIx) -> Option<&Relationship<E>> {
        self.rels.get(id.0).and_then(|r| r.as_ref().map(|r| &**r))
    }

    pub fn rel_mut(&mut self, id: RelationshipIx) -> Option<RelationshipMut<N, E>> {
        Some(RelationshipMut {
            rel: id,
            graph: self,
        })
    }

    pub fn iter_all_nodes(&self) -> Vec<&Node<N>> {
        self.nodes
            .iter()
            .filter(|n| n.is_some())
            .map(|n| n.as_ref().map(|n| &**n).unwrap())
            .collect::<Vec<&Node<_>>>()
    }

    pub fn iter_all_rels(&self) -> Vec<&Relationship<E>> {
        self.rels
            .iter()
            .filter(|r| r.is_some())
            .map(|r| r.as_ref().map(|r| &**r).unwrap())
            .collect::<Vec<&Relationship<_>>>()
    }

    //    pub fn dump(&self) {
    // println!("");
    // self.nodes
    // .iter()
    // .filter(|n| n.is_some())
    // .map(|n| {
    // println!("{:?}", n.as_ref().unwrap());
    // })
    // .last();
    //
    // self.rels
    // .iter()
    // .filter(|r| r.is_some())
    // .map(|r| {
    // println!("{:?}", r.as_ref().unwrap());
    // })
    // .last();
    // }
    //
    //
}
impl<N: fmt::Debug + fmt::Display, E: fmt::Debug + fmt::Display> Graph<N, E> {
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
                                      r.weight));
            })
            .last();
        dot.push_str("}\n");
        dot
    }
}


impl<N> Node<N> {
    pub fn get_id(&self) -> NodeIx {
        NodeIx(self.id)
    }

    pub fn has_rel(&self) -> bool {
        self.next_start_rel.or(self.next_end_rel).is_some()
    }
}


impl<E> Relationship<E> {
    pub fn get_id(&self) -> RelationshipIx {
        RelationshipIx(self.id)
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
    let mut g = Graph::<i32, i32>::new();
    let mut v = g.create_node(233);
    let mut x = g.create_node(666);
    let mut u = g.create_node(999);
    let w = g.create_node(999);
    let q = g.create_node(555);

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
