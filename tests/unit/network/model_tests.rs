/// Unit tests for network data structure
use crate::network::model::*;

#[path = "../utils.rs"]
mod utils;

use crate::network::model::model_tests::utils::*;

#[test]
fn test_network_empty_on_beginning() {
    let net: Network<usize, f64> = Network::new(false);

    check_network(net, 0, 0, vec![], vec![]);
}

#[test]
fn test_node_inserted() {
    let mut net: Network<usize, f64> = Network::new(false);

    net.add_node(0);

    check_network(net, 1, 0, vec![0], vec![]);
}

#[test]
fn test_multiple_node_insertion() {
    let mut net: Network<usize, f64> = Network::new(false);

    net.add_node(0);
    net.add_node(0);

    check_network(net, 1, 0, vec![0], vec![]);
}

#[test]
fn test_edge_inserted_directed() {
    let mut net: Network<usize, f64> = Network::new(true);

    net.add_node(0);
    net.add_node(1);

    net.add_edge(0, 1, 10.0);

    check_network(net, 2, 1, vec![0, 1], vec![(0, 1, vec![&10.0])]);
}

#[test]
fn test_edge_inserted_undirected() {
    let mut net: Network<usize, f64> = Network::new(false);

    net.add_node(0);
    net.add_node(1);

    net.add_edge(0, 1, 10.0);

    check_network(
        net,
        2,
        1,
        vec![0, 1],
        vec![(0, 1, vec![&10.0]), (1, 0, vec![&10.0])],
    );
}

#[test]
fn test_edge_inserted_without_adding_nodes_directed() {
    let mut net: Network<usize, f64> = Network::new(true);

    net.add_edge(0, 1, 10.0);

    check_network(net, 2, 1, vec![0, 1], vec![(0, 1, vec![&10.0])]);
}

#[test]
fn test_edge_inserted_without_adding_nodes_undirected() {
    let mut net: Network<usize, f64> = Network::new(false);

    net.add_edge(0, 1, 10.0);

    check_network(
        net,
        2,
        1,
        vec![0, 1],
        vec![(0, 1, vec![&10.0]), (1, 0, vec![&10.0])],
    );
}

#[test]
fn test_edge_not_present() {
    let net: Network<&str, usize> = Network::new(false);

    assert!(
        net.edge_data("A", "B").is_none(),
        "There should not be an edge between A and B"
    );
}

#[test]
fn test_edge_present_directed() {
    let mut net: Network<&str, usize> = Network::new(true);

    net.add_edge("A", "B", 1);

    assert!(
        net.edge_data("A", "B").is_some(),
        "There should be an edge between A and B"
    );

    assert!(
        net.edge_data("B", "A").is_none(),
        "There should not be an edge between A and B"
    );
}

#[test]
fn test_edge_present_undirected() {
    let mut net: Network<&str, usize> = Network::new(false);

    net.add_edge("A", "B", 1);

    assert!(
        net.edge_data("A", "B").is_some(),
        "There should be an edge between A and B"
    );

    assert!(
        net.edge_data("B", "A").is_some(),
        "There should be an edge between A and B"
    );
}
