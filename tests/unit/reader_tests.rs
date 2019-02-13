/// Unit tests for network file reader
use std::path::PathBuf;

use crate::network::Network;
use crate::reader::*;

#[test]
fn test_reading_non_existing_file() {
    let reader = EdgeListReader::new(',');
    let path: PathBuf = "/tmp/non-existing-file.txt".parse().unwrap();

    let res = reader.read(path);
    assert_eq!(res.unwrap_err(), ReaderError::FileNotFound);
}

#[test]
fn test_reading_simple_file() {
    let reader = EdgeListReader::new(',');
    let path: PathBuf = "resources/simple-net.txt".parse().unwrap();

    let res = reader.read(path);
    assert!(res.is_ok(), "Should read simple file");

    let net: Network<usize, f64> = res.unwrap();
    assert_eq!(net.num_nodes(), 3, "Should be 3 nodes");
    assert_eq!(net.num_edges(), 3, "Should be 3 edges");

    assert!(net.nodes().contains(&&0), "Should contain node: 0");
    assert!(net.nodes().contains(&&1), "Should contain node: 1");
    assert!(net.nodes().contains(&&2), "Should contain node: 2");

    assert!(net.edges().contains(&&(0, 1)), "Should contain edge: 0-1");
    assert!(net.edges().contains(&&(1, 2)), "Should contain node: 1-2");
    assert!(net.edges().contains(&&(2, 0)), "Should contain node: 2-0");

    assert_eq!(
        *net.edge_data(0, 1).unwrap(),
        16.0,
        "Edge 0-1 should have data equal to 16.0"
    );
    assert_eq!(
        *net.edge_data(1, 2).unwrap(),
        18.0,
        "Edge 1-2 should have data equal to 18.0"
    );
    assert_eq!(
        *net.edge_data(2, 0).unwrap(),
        20.0,
        "Edge 2-0 should have data equal to 20.0"
    );
}

#[test]
fn test_malformed_not_enough_elements() {
    let reader = EdgeListReader::new(',');
    let path: PathBuf = "resources/malformed-not-enough-elements.txt"
        .parse()
        .unwrap();

    let res = reader.read(path);
    assert_eq!(
        res.unwrap_err(),
        ReaderError::MalformedFileFormat("Not enough elements".to_string())
    );
}

#[test]
fn test_malformed_int_parse_error() {
    let reader = EdgeListReader::new(',');
    let path: PathBuf = "resources/malformed-int-parse-error.txt".parse().unwrap();

    let res = reader.read(path);
    assert_eq!(
        res.unwrap_err(),
        ReaderError::MalformedFileFormat("Couldn't parse int".to_string())
    );
}

#[test]
fn test_malformed_float_parse_error() {
    let reader = EdgeListReader::new(',');
    let path: PathBuf = "resources/malformed-float-parse-error.txt".parse().unwrap();

    let res = reader.read(path);
    assert_eq!(
        res.unwrap_err(),
        ReaderError::MalformedFileFormat("Couldn't parse float".to_string())
    );
}

#[test]
fn test_fb_forum() {
    let reader = EdgeListReader::new(',');
    let path: PathBuf = "resources/fb-forum.txt".parse().unwrap();

    let res = reader.read(path);
    assert!(res.is_ok(), "Should read fb-forum");

    let net: Network<usize, f64> = res.unwrap();
    assert_eq!(net.num_nodes(), 899, "Should be 899 nodes");
    assert_eq!(net.num_edges(), 7089, "Should be 7089 edges");
}
