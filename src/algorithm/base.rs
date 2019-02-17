/// Base code for all embedding algorithms
use crate::embedding::model::{Embedding, EmbeddingKey};
use crate::network::model::{Network, NetworkNode};

trait BaseAlgorithm {
    fn embed<N: NetworkNode + EmbeddingKey, E: Copy>(&self, net: Network<N, E>) -> Embedding<N>;
}
