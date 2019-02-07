/// Unit tests for network data structure

use crate::network::*;

#[test]
fn test_network_empty_on_beginning() {
    let net: Network<&str, &str> = Network::new();

    assert_eq!(net.graph.node_count(), 0, "Number of nodes in graph should be zero");
    assert_eq!(net.graph.edge_count(), 0, "Number of edges in graph should be zero");
    assert!(net.nodes.is_empty(), "Nodes map should be empty");
    assert!(net.edges.is_empty(), "Edges map should be empty");
}

#[test]
fn test_node_inserted() {
    let mut net: Network<&str, &str> = Network::new();

    net.add_node("A");

    assert!(net.nodes.contains_key("A"), "Should contain A node");
}
