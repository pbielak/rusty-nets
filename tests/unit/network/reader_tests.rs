/// Unit tests for network file reader
use crate::network::reader::*;

#[path = "../utils.rs"]
mod utils;

use crate::network::reader::reader_tests::utils::*;

#[test]
fn test_reading_non_existing_file() {
    check_reader_error("/tmp/non-existing-file.txt", ReaderError::FileNotFound);
}

#[test]
fn test_malformed_not_enough_elements() {
    check_reader_error(
        "resources/nets/malformed-not-enough-elements.txt",
        ReaderError::MalformedFileFormat("Not enough elements".to_string()),
    );
}

#[test]
fn test_malformed_int_parse_error() {
    check_reader_error(
        "resources/nets/malformed-int-parse-error.txt",
        ReaderError::MalformedFileFormat("Couldn't parse int".to_string()),
    );
}

#[test]
fn test_malformed_float_parse_error() {
    check_reader_error(
        "resources/nets/malformed-float-parse-error.txt",
        ReaderError::MalformedFileFormat("Couldn't parse float".to_string()),
    );
}

#[test]
fn test_reading_simple_file_directed() {
    let net = read_net("resources/nets/simple-net.txt", true);
    check_network(
        net,
        3,
        3,
        vec![0, 1, 2],
        vec![
            (0, 1, vec![&16.0]),
            (1, 2, vec![&18.0]),
            (2, 0, vec![&20.0]),
        ],
    );
}

#[test]
fn test_reading_simple_file_undirected() {
    let net = read_net("resources/nets/simple-net.txt", false);
    check_network(
        net,
        3,
        3,
        vec![0, 1, 2],
        vec![
            (0, 1, vec![&16.0]),
            (1, 2, vec![&18.0]),
            (2, 0, vec![&20.0]),
            (1, 0, vec![&16.0]),
            (2, 1, vec![&18.0]),
            (0, 2, vec![&20.0]),
        ],
    );
}

#[test]
fn test_fb_forum_directed() {
    let net = read_net("resources/nets/fb-forum.txt", true);
    check_network(
        net,
        899,
        33720,
        // NOTE(pbielak): Leaving empty Vecs here as it would be too much data.
        // Just want test the number of nodes and number of edges.
        vec![],
        vec![],
    );
}

#[test]
fn test_fb_forum_undirected() {
    let net = read_net("resources/nets/fb-forum.txt", false);
    check_network(
        net,
        899,
        33720,
        // NOTE(pbielak): Leaving empty Vecs here as it would be too much data.
        // Just want test the number of nodes and number of edges.
        vec![],
        vec![],
    );
}
