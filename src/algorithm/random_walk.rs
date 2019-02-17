/// Random walkers
use rand::prelude::*;

use crate::network::model::{Network, NetworkNode};

trait RandomWalker {
    fn walk<N: NetworkNode, E: Copy>(&mut self, net: &Network<N, E>, start_node: N) -> Vec<N>;
}

pub struct NonBiasedRandomWalker {
    walks_per_node: usize,
    walk_length: usize,
    rng: StdRng,
}

impl NonBiasedRandomWalker {
    pub fn new(walks_per_node: usize, walk_length: usize) -> NonBiasedRandomWalker {
        NonBiasedRandomWalker {
            walks_per_node,
            walk_length,
            rng: StdRng::from_entropy(),
        }
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.rng = StdRng::seed_from_u64(seed);
    }
}

impl RandomWalker for NonBiasedRandomWalker {
    fn walk<N: NetworkNode, E: Copy>(&mut self, net: &Network<N, E>, start_node: N) -> Vec<N> {
        let mut walk: Vec<N> = vec![];

        walk.push(start_node);

        while walk.len() < self.walk_length {
            match net.neighbours_of(*walk.last().unwrap()) {
                None => break,
                Some(neighbours) => {
                    let idx = self.rng.gen_range(0, neighbours.len());
                    walk.push(*neighbours[idx]);
                }
            }
        }

        walk
    }
}

#[cfg(test)]
#[path = "../../tests/unit/algorithm/random_walker_tests.rs"]
mod random_walker_tests;
