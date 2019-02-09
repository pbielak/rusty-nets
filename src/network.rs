/// Network data structure
use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

use petgraph::Graph;
use petgraph::graph::{EdgeIndex, NodeIndex};


#[derive(Default)]
pub struct Network<N, E>
    where N: Eq + Hash + Copy,
          E: Eq + Hash + Copy
{
    graph: Graph<N, E>,
    nodes: HashMap<N, NodeIndex<u32>>,
    edges: HashMap<(N, N), EdgeIndex<u32>>,
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

    pub fn add_node(&mut self, node: N) -> NodeIndex<u32> {
        match self.nodes.get(&node) {
            Some(nx) => *nx,
            None => {
                let nx = self.graph.add_node(node);
                self.nodes.insert(node, nx);

                nx
            }
        }
    }

    pub fn add_edge(&mut self, from: N, to: N, edge_data: E) -> EdgeIndex<u32> {
        match self.edges.get(&(from, to)) {
            Some(ex) => *ex,
            None => {
                let from_nx = match self.nodes.get(&from) {
                    Some(from_nx) => *from_nx,
                    None => self.add_node(from)
                };
                let to_nx = match self.nodes.get(&to) {
                    Some(to_nx) => *to_nx,
                    None => self.add_node(to)
                };

                let ex = self.graph.add_edge(from_nx, to_nx, edge_data);
                self.edges.insert((from, to), ex);

                ex
            }
        }
    }

    pub fn get_edge_data(&self, from: N, to: N) -> Option<&E> {
        match self.edges.get(&(from, to)) {
            Some(ex) => self.graph.edge_weight(*ex),
            None => None
        }
    }
}


#[cfg(test)]
#[path = "../tests/unit/network_tests.rs"]
mod network_tests;
