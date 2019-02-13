/// Unit tests for network file reader
use std::path::PathBuf;

use crate::embedding::model::Embedding;
use crate::embedding::reader::*;

#[test]
fn test_reading_non_existing_file() {
    let reader = W2VEmbeddingVectorsReader::new();
    let path: PathBuf = "/tmp/non-existing-file.txt".parse().unwrap();

    let res = reader.read(path);
    assert_eq!(res.unwrap_err(), ReaderError::FileNotFound);
}

#[test]
fn test_reading_simple_file() {
    let reader = W2VEmbeddingVectorsReader::new();
    let path: PathBuf = "resources/embs/simple-embedding.txt".parse().unwrap();

    let res = reader.read(path);
    assert!(res.is_ok(), "Should read simple embedding");

    let emb: Embedding<usize> = res.unwrap();
    assert_eq!(emb.dim(), 3, "Embedding should have dimension 3");
    assert_eq!(emb.keys().len(), 2, "Embedding should contain 2 nodes");

    assert!(emb.keys().contains(&&0), "Should contain node: 0");
    assert!(emb.keys().contains(&&1), "Should contain node: 1");

    assert_eq!(
        *emb.get_vector(0).unwrap(),
        [1.0, 2.0, 3.0],
        "Vector for 0 node should be 1.0 2.0 3.0"
    );
    assert_eq!(
        *emb.get_vector(1).unwrap(),
        [3.0, 2.0, 1.0],
        "Vector for 1 node should be 3.0 2.0 1.0"
    );
}

#[test]
fn test_malformed_header() {
    let reader = W2VEmbeddingVectorsReader::new();
    let path: PathBuf = "resources/embs/malformed-header.txt".parse().unwrap();

    let res = reader.read(path);
    assert_eq!(
        res.unwrap_err(),
        ReaderError::MalformedFileFormat(
            "Header should contain 2 elements: number of nodes and embedding dimension".to_string()
        )
    );
}

#[test]
fn test_malformed_line() {
    let reader = W2VEmbeddingVectorsReader::new();
    let path: PathBuf = "resources/embs/malformed-line.txt".parse().unwrap();

    let res = reader.read(path);
    assert_eq!(
        res.unwrap_err(),
        ReaderError::MalformedFileFormat("Not enough elements".to_string())
    );
}

#[test]
fn test_malformed_vector() {
    let reader = W2VEmbeddingVectorsReader::new();
    let path: PathBuf = "resources/embs/malformed-vector.txt".parse().unwrap();

    let res = reader.read(path);
    assert_eq!(
        res.unwrap_err(),
        ReaderError::MalformedFileFormat("Couldn\'t parse float".to_string())
    );
}

#[test]
fn test_read_fb_forum_embedding() {
    let reader = W2VEmbeddingVectorsReader::new();
    let path: PathBuf = "resources/embs/fb-forum-embedding.txt".parse().unwrap();

    let res = reader.read(path);
    assert!(res.is_ok(), "Should read fb-forum embedding");

    let emb: Embedding<usize> = res.unwrap();
    assert_eq!(emb.dim(), 128, "Embedding should have dimension 128");
    assert_eq!(emb.keys().len(), 390, "Embedding should contain 390 nodes");
}
