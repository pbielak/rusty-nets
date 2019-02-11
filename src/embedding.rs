/// Embedding data structure
use std::cmp::Eq;
use std::collections::HashMap;
use std::default::Default;
use std::fmt::Debug;
use std::hash::Hash;

use ndarray::prelude::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    WrongEmbeddingDimension(String),
    VectorNotFound(String),
}

pub trait EmbeddingKey: Eq + Hash + Copy + Default + Debug {}

impl<T> EmbeddingKey for T where T: Eq + Hash + Copy + Default + Debug {}

#[derive(Debug, Default)]
pub struct Embedding<K: EmbeddingKey> {
    emb_dim: usize,
    emb_vectors: HashMap<K, Array1<f64>>,
}

impl<K: EmbeddingKey> Embedding<K> {
    pub fn new(dim: usize) -> Embedding<K> {
        Embedding {
            emb_dim: dim,
            emb_vectors: HashMap::new(),
        }
    }

    pub fn add_vector(&mut self, key: K, vector: &[f64]) -> Result<(), Error> {
        let vector: Array1<f64> = Array1::from_vec(Vec::from(vector));

        if vector.dim() != self.emb_dim {
            return Err(Error::WrongEmbeddingDimension(format!(
                "Expected: {} Got: {}",
                self.emb_dim,
                vector.dim()
            )));
        }

        self.emb_vectors.insert(key, vector);

        Ok(())
    }

    pub fn get_vector(&self, key: K) -> Result<&[f64], Error> {
        match self.emb_vectors.get(&key) {
            Some(v) => Ok(v.as_slice().unwrap()),
            None => Err(Error::VectorNotFound(format!(
                "No vector found for {:?}",
                key
            ))),
        }
    }

    pub fn dim(&self) -> usize {
        self.emb_dim
    }

    pub fn keys(&self) -> Vec<&K> {
        self.emb_vectors.keys().collect()
    }
}

#[cfg(test)]
#[path = "../tests/unit/embedding_tests.rs"]
mod embedding_tests;
