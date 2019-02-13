/// Embedding file reader
use std::fs;
use std::io;
use std::io::BufRead;
use std::num;
use std::path::PathBuf;

use crate::embedding::model::*;

#[derive(Debug, PartialEq)]
pub enum ReaderError {
    FileNotFound,
    MalformedFileFormat(String),
}

impl From<io::Error> for ReaderError {
    fn from(e: io::Error) -> Self {
        match e.kind() {
            io::ErrorKind::NotFound => ReaderError::FileNotFound,
            _ => panic!(format!("Unknown error occurred: {:?}", e.kind())),
        }
    }
}

impl From<num::ParseIntError> for ReaderError {
    fn from(_: num::ParseIntError) -> Self {
        ReaderError::MalformedFileFormat("Couldn't parse int".to_string())
    }
}

impl From<num::ParseFloatError> for ReaderError {
    fn from(_: num::ParseFloatError) -> Self {
        ReaderError::MalformedFileFormat("Couldn't parse float".to_string())
    }
}

pub trait EmbeddingReader {
    fn read(&self, file: PathBuf) -> Result<Embedding<usize>, ReaderError>;
}

pub struct W2VEmbeddingVectorsReader {}

impl W2VEmbeddingVectorsReader {
    pub fn new() -> W2VEmbeddingVectorsReader {
        W2VEmbeddingVectorsReader {}
    }
}

impl EmbeddingReader for W2VEmbeddingVectorsReader {
    fn read(&self, file: PathBuf) -> Result<Embedding<usize>, ReaderError> {
        let f = fs::File::open(file)?;
        let reader = io::BufReader::new(&f);

        let mut lines = reader.lines();

        let header_raw: String = lines.next().unwrap().unwrap();
        let header: Vec<&str> = header_raw.split(' ').collect();

        if header.len() != 2 {
            return Err(ReaderError::MalformedFileFormat(
                "Header should contain 2 elements: number of nodes and embedding dimension"
                    .to_string(),
            ));
        }

        let dim: usize = header[1].parse()?;

        let mut emb = Embedding::new(dim);

        for line in lines {
            let emb_map_raw: String = line?;
            let emb_map: Vec<&str> = emb_map_raw.split(' ').collect();

            if emb_map.len() != (dim + 1) {
                return Err(ReaderError::MalformedFileFormat(
                    "Not enough elements".to_string(),
                ));
            }

            let node: usize = emb_map[0].parse()?;
            let mut vector: Vec<f64> = Vec::new();
            for v in emb_map[1..].iter() {
                vector.push(v.parse()?);
            }

            let _ = emb.add_vector(node, vector.as_slice());
        }

        Ok(emb)
    }
}

#[cfg(test)]
#[path = "../../tests/unit/embedding/reader_tests.rs"]
mod reader_tests;
