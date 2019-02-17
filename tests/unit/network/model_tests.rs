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

#[test]
fn test_no_neighbours_empty() {
    let net: Network<&str, f64> = Network::new(false);

    assert!(
        net.neighbours_of("A").is_none(),
        "There should not be any neighbours in empty net."
    )
}

#[test]
fn test_no_neighbours_one_node() {
    let mut net: Network<&str, f64> = Network::new(false);

    net.add_node("A");

    assert!(
        net.neighbours_of("A").is_none(),
        "There should not be any neighbours for single node."
    )
}

#[test]
fn test_neighbours_found_directed() {
    let mut net: Network<&str, f64> = Network::new(true);

    net.add_node("A");
    net.add_node("B");
    net.add_node("C");

    net.add_edge("A", "B", 1.0);
    net.add_edge("A", "C", 2.0);

    let neighbours: Vec<&&str> = net.neighbours_of("A").unwrap();

    assert_eq!(neighbours.len(), 2, "There should be two neighbours: B, C.");
    assert!(neighbours.contains(&&"B"), "Should contain B node.");
    assert!(neighbours.contains(&&"C"), "Should contain C node.");
}

#[test]
fn test_neighbours_found_undirected() {
    let mut net: Network<&str, f64> = Network::new(false);

    net.add_node("A");
    net.add_node("B");
    net.add_node("C");

    net.add_edge("A", "B", 1.0);
    net.add_edge("A", "C", 2.0);

    let neighbours: Vec<&&str> = net.neighbours_of("A").unwrap();

    assert_eq!(neighbours.len(), 2, "There should be two neighbours: B, C.");
    assert!(neighbours.contains(&&"B"), "Should contain B node.");
    assert!(neighbours.contains(&&"C"), "Should contain C node.");
}

#[test]
fn test_no_neighbours_dst_node_directed() {
    let mut net: Network<&str, f64> = Network::new(true);

    net.add_node("A");
    net.add_node("B");

    net.add_edge("A", "B", 1.0);

    assert!(
        net.neighbours_of("B").is_none(),
        "There should not be any neighbours for dst node (if directed net)."
    )
}

#[test]
fn test_neighbours_found_dst_node_undirected() {
    let mut net: Network<&str, f64> = Network::new(false);

    net.add_node("A");
    net.add_node("B");

    net.add_edge("A", "B", 1.0);

    let neighbours: Vec<&&str> = net.neighbours_of("B").unwrap();

    assert_eq!(neighbours.len(), 1, "There should be one neighbour: A.");
    assert!(neighbours.contains(&&"A"), "Should contain A node.");
}
