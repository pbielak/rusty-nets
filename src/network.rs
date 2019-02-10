/// Network data structure
use std::cmp::Eq;
use std::collections::HashMap;
use std::default::Default;
use std::hash::Hash;

use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::Graph;

pub trait NetworkNode: Eq + Hash + Copy + Default {}

impl<T> NetworkNode for T where T: Eq + Hash + Copy + Default {}

#[derive(Debug, Default)]
pub struct Network<N: NetworkNode, E> {
    graph: Graph<N, E>,
    nodes: HashMap<N, NodeIndex<u32>>,
    edges: HashMap<(N, N), EdgeIndex<u32>>,
}

impl<N: NetworkNode, E> Network<N, E> {
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
                    None => self.add_node(from),
                };
                let to_nx = match self.nodes.get(&to) {
                    Some(to_nx) => *to_nx,
                    None => self.add_node(to),
                };

                let ex = self.graph.add_edge(from_nx, to_nx, edge_data);
                self.edges.insert((from, to), ex);

                ex
            }
        }
    }

    pub fn edge_data(&self, from: N, to: N) -> Option<&E> {
        match self.edges.get(&(from, to)) {
            Some(ex) => self.graph.edge_weight(*ex),
            None => None,
        }
    }

    pub fn num_nodes(&self) -> usize {
        self.graph.node_count()
    }

    pub fn num_edges(&self) -> usize {
        self.graph.edge_count()
    }

    pub fn nodes(&self) -> Vec<&N> {
        self.nodes.keys().collect()
    }

    pub fn edges(&self) -> Vec<&(N, N)> {
        self.edges.keys().collect()
    }
}

#[cfg(test)]
#[path = "../tests/unit/network_tests.rs"]
mod network_tests;
