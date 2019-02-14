/// Network data structure
use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::Graph;

pub trait NetworkNode: Eq + Hash + Copy {}

impl<T> NetworkNode for T where T: Eq + Hash + Copy {}

#[derive(Debug)]
pub struct Network<N: NetworkNode, E: Copy> {
    graph: Graph<N, E>,
    nodes: HashMap<N, NodeIndex<u32>>,
    edges: HashMap<(N, N), Vec<EdgeIndex<u32>>>,
    is_directed: bool,
}

impl<N: NetworkNode, E: Copy> Network<N, E> {
    pub fn new(directed: bool) -> Network<N, E> {
        Network {
            graph: Graph::new(),
            nodes: HashMap::new(),
            edges: HashMap::new(),
            is_directed: directed,
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
        let ex = self._add_edge(from, to, edge_data);

        if !self.is_directed {
            self._add_edge(to, from, edge_data);
        }

        ex
    }

    fn _add_edge(&mut self, from: N, to: N, edge_data: E) -> EdgeIndex<u32> {
        let from_nx = match self.nodes.get(&from) {
            Some(from_nx) => *from_nx,
            None => self.add_node(from),
        };
        let to_nx = match self.nodes.get(&to) {
            Some(to_nx) => *to_nx,
            None => self.add_node(to),
        };

        let ex = self.graph.add_edge(from_nx, to_nx, edge_data);

        match self.edges.get(&(from, to)) {
            None => {
                self.edges.insert((from, to), vec![ex]);
            }
            Some(v) => {
                let mut vec = vec![ex];
                vec.extend(v);
                self.edges.insert((from, to), vec);
            }
        }

        ex
    }

    pub fn edge_data(&self, from: N, to: N) -> Option<Vec<&E>> {
        match self.edges.get(&(from, to)) {
            Some(ex) => ex.iter().map(|e| self.graph.edge_weight(*e)).collect(),
            None => None,
        }
    }

    pub fn num_nodes(&self) -> usize {
        self.graph.node_count()
    }

    pub fn num_edges(&self) -> usize {
        let mut num_edges = self.graph.edge_count();
        if !self.is_directed {
            num_edges /= 2;
        }

        num_edges
    }

    pub fn nodes(&self) -> Vec<&N> {
        self.nodes.keys().collect()
    }

    pub fn edges(&self) -> Vec<&(N, N)> {
        self.edges.keys().collect()
    }
}

#[cfg(test)]
#[path = "../../tests/unit/network/model_tests.rs"]
mod model_tests;
