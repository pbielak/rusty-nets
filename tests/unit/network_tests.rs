/// Unit tests for network data structure
use crate::network::model::*;

#[test]
fn test_network_empty_on_beginning() {
    let net: Network<&str, &str> = Network::new();

    assert_eq!(
        net.num_nodes(),
        0,
        "Number of nodes in graph should be zero"
    );
    assert_eq!(
        net.num_edges(),
        0,
        "Number of edges in graph should be zero"
    );
}

#[test]
fn test_node_inserted() {
    let mut net: Network<&str, &str> = Network::new();

    net.add_node("A");

    assert_eq!(net.num_nodes(), 1, "Number of nodes in graph should be one");
    assert_eq!(
        net.num_edges(),
        0,
        "Number of edges in graph should be zero"
    );
    assert!(net.nodes().contains(&&"A"), "Should contain A node");
}

#[test]
fn test_multiple_node_insertion() {
    let mut net: Network<&str, &str> = Network::new();

    net.add_node("A");
    net.add_node("A");

    assert_eq!(net.num_nodes(), 1, "Number of nodes in graph should be one");
    assert_eq!(
        net.num_edges(),
        0,
        "Number of edges in graph should be zero"
    );
    assert!(net.nodes().contains(&&"A"), "Should contain A node");
}

#[test]
fn test_edge_inserted() {
    let mut net: Network<&str, usize> = Network::new();

    net.add_node("A");
    net.add_node("B");

    net.add_edge("A", "B", 1);

    assert_eq!(net.num_nodes(), 2, "Number of nodes in graph should be two");
    assert_eq!(net.num_edges(), 1, "Number of edges in graph should be one");
    assert!(net.nodes().contains(&&"A"), "Should contain A node");
    assert!(net.nodes().contains(&&"B"), "Should contain B node");
    assert!(
        net.edges().contains(&&("A", "B")),
        "Should contain A-B edge"
    );

    let weight = net.edge_data("A", "B").unwrap();
    assert_eq!(*weight, 1, "Edge A-B should have weight 1");
}

#[test]
fn test_edge_inserted_without_adding_nodes() {
    let mut net: Network<&str, usize> = Network::new();

    net.add_edge("A", "B", 1);

    assert_eq!(net.num_nodes(), 2, "Number of nodes in graph should be two");
    assert_eq!(net.num_edges(), 1, "Number of edges in graph should be one");
    assert!(net.nodes().contains(&&"A"), "Should contain A node");
    assert!(net.nodes().contains(&&"B"), "Should contain B node");
    assert!(
        net.edges().contains(&&("A", "B")),
        "Should contain A-B edge"
    );

    let weight = net.edge_data("A", "B").unwrap();
    assert_eq!(*weight, 1, "Edge A-B should have weight 1");
}

#[test]
fn test_edge_not_present() {
    let net: Network<&str, usize> = Network::new();

    assert!(
        net.edge_data("A", "B").is_none(),
        "There should not be an edge between A and B"
    );
}

#[test]
fn test_edge_present() {
    let mut net: Network<&str, usize> = Network::new();

    net.add_edge("A", "B", 1);

    assert!(
        net.edge_data("A", "B").is_some(),
        "There should not be an edge between A and B"
    );
}
