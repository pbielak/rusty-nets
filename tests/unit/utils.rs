/// Test utils
use std::path::PathBuf;

use crate::network::model::Network;
use crate::network::reader::*;

#[allow(dead_code)]
pub fn check_reader_error(file: &str, expected_err: ReaderError) {
    let reader = EdgeListReader::new(',', false);
    let path: PathBuf = file.parse().unwrap();

    let res = reader.read(path);
    assert_eq!(res.unwrap_err(), expected_err);
}

#[allow(dead_code)]
pub fn check_network(
    net: Network<usize, f64>,
    num_nodes: usize,
    num_edges: usize,
    nodes: Vec<usize>,
    edges: Vec<(usize, usize, Vec<&f64>)>,
) {
    assert_eq!(net.num_nodes(), num_nodes);
    assert_eq!(net.num_edges(), num_edges);

    for node in nodes {
        assert!(net.nodes().contains(&&node));
    }

    for edge in edges {
        let (from, to, data) = edge;
        assert!(net.edges().contains(&&(from, to)));
        assert_eq!(net.edge_data(from, to).unwrap(), data);
    }
}

#[allow(dead_code)]
pub fn read_net(file: &str, directed: bool) -> Network<usize, f64> {
    let reader = EdgeListReader::new(',', directed);
    let path: PathBuf = file.parse().unwrap();

    let res = reader.read(path);
    assert!(res.is_ok(), "Should read network");

    res.unwrap()
}
