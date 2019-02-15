/// Base code for all embedding algorithms
use crate::embedding::model::{Embedding, EmbeddingKey};
use crate::network::model::{Network, NetworkNode};

trait BaseAlgorithm {
    fn embed<N, E>(net: Network<N, E>) -> Embedding<N>
    where
        N: NetworkNode + EmbeddingKey,
        E: Copy;
}
