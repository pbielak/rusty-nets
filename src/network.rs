/// Network data structure
extern crate petgraph;

use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

use petgraph::Graph;
use petgraph::graph::{EdgeIndex, NodeIndex};


pub struct Network<N, E> {
    graph: Graph<N, E>,
    nodes: HashMap<N, NodeIndex<u32>>,
    edges: HashMap<E, EdgeIndex<u32>>,
}

impl <N, E> Network<N, E>
    where N: Eq + Hash + Copy,
          E: Eq + Hash + Copy
{
    pub fn new() -> Network<N, E> {
        Network {
            graph: Graph::new(),
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: N) {
        let nx = self.graph.add_node(node);
        self.nodes.insert(node, nx);
    }
}


#[cfg(test)]
#[path = "../tests/unit/network_tests.rs"]
mod network_tests;
