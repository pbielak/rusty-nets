/// Unit tests for embedding data structure
use crate::embedding::model::*;

#[test]
fn test_add_vector_wrong_dim() {
    let dim: usize = 3;
    let mut emb: Embedding<&str> = Embedding::new(dim);

    let res = emb.add_vector("A", &[1.0, 2.0]);
    assert_eq!(
        res.unwrap_err(),
        EmbeddingError::WrongEmbeddingDimension("Expected: 3 Got: 2".to_string())
    )
}

#[test]
fn test_add_vector_proper_dim() {
    let dim: usize = 3;
    let mut emb: Embedding<&str> = Embedding::new(dim);

    let res = emb.add_vector("A", &[1.0, 2.0, 3.0]);
    assert!(res.is_ok(), "Adding embedding vector should succeed");

    let vector = emb.emb_vectors.get("A").unwrap().as_slice().unwrap();
    assert_eq!(vector, &[1.0, 2.0, 3.0], "Embedding vector should be added");
}

#[test]
fn test_get_non_existing_vector() {
    let dim: usize = 3;
    let emb: Embedding<&str> = Embedding::new(dim);

    let vector = emb.get_vector("A");
    assert_eq!(
        vector.unwrap_err(),
        EmbeddingError::VectorNotFound("No vector found for \"A\"".to_string())
    );
}

#[test]
fn test_get_existing_vector() {
    let dim: usize = 3;
    let mut emb: Embedding<&str> = Embedding::new(dim);

    let res = emb.add_vector("A", &[1.0, 2.0, 3.0]);
    assert!(res.is_ok(), "Adding embedding vector should succeed");

    let vector = emb.get_vector("A");
    assert!(vector.is_ok(), "Vector should be present");
    assert_eq!(
        vector.unwrap(),
        &[1.0, 2.0, 3.0],
        "Embedding vector should be the same as added"
    );
}

#[test]
fn test_get_dim() {
    let dim: usize = 3;
    let emb: Embedding<&str> = Embedding::new(dim);

    assert_eq!(
        emb.dim(),
        dim,
        "Embedding dimension should be same as given"
    );
}

#[test]
fn test_get_keys_empty() {
    let dim: usize = 3;
    let emb: Embedding<&str> = Embedding::new(dim);

    assert!(emb.keys().is_empty(), "There should be no embedding keys");
}

#[test]
fn test_get_keys_nonempty() {
    let dim: usize = 3;
    let mut emb: Embedding<&str> = Embedding::new(dim);

    emb.add_vector("A", &[1.0, 2.0, 3.0])
        .expect("Couldn't add \"A\" vector");
    emb.add_vector("B", &[3.0, 2.0, 1.0])
        .expect("Couldn't add \"B\" vector");

    let keys: Vec<&&str> = emb.keys();

    assert_eq!(keys.len(), 2, "Two embedding keys should be present");

    assert!(keys.contains(&&"A"), "Vector \"A\" should be added");
    assert!(keys.contains(&&"B"), "Vector \"B\" should be added");
}
