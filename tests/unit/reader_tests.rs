/// Unit tests for network file reader
use std::path::PathBuf;

use crate::reader::*;

#[test]
fn test_reading_non_existing_file() {
    let reader = EdgeListReader::new(',');
    let path: PathBuf = "/tmp/non-existing-file.txt".parse().unwrap();

    let res = reader.read(path);
    assert_eq!(res.unwrap_err(), Error::FileNotFound);
}

#[test]
fn test_reading_simple_file() {
    let reader = EdgeListReader::new(',');
    let path: PathBuf = "resources/simple-net.txt".parse().unwrap();

    let res = reader.read(path);
    assert!(res.is_ok(), "Should read simple file");

    let net = res.unwrap();
    assert_eq!(
        *net.get_edge_data(0, 1).unwrap(),
        16.0,
        "Edge 0-1 should have data equal to 16.0"
    );
    assert_eq!(
        *net.get_edge_data(1, 2).unwrap(),
        18.0,
        "Edge 1-2 should have data equal to 18.0"
    );
    assert_eq!(
        *net.get_edge_data(2, 0).unwrap(),
        20.0,
        "Edge 2-0 should have data equal to 20.0"
    );
}
