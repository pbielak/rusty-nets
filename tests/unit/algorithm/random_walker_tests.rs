/// Unit tests for random walkers
use crate::algorithm::random_walk::*;
use crate::network::model::Network;

fn run_nonbiased_rw(
    net: &Network<&str, f64>,
    seed: u64,
    walk_per_node: usize,
    walk_length: usize,
    start_node: &str,
    expected_walk: &[&str],
) {
    let mut rw = NonBiasedRandomWalker::new(walk_per_node, walk_length);
    rw.set_seed(seed);

    let walk = rw.walk(&net, start_node);

    assert_eq!(walk, expected_walk);
}

#[test]
fn test_nonbiased_rw_single_walk_single_node() {
    let mut net: Network<&str, f64> = Network::new(true);

    net.add_node("A");

    run_nonbiased_rw(&net, 0, 1, 4, "A", &["A"]);
}

#[test]
fn test_nonbiased_rw_single_walk_multi_node_nonconnected() {
    let mut net: Network<&str, f64> = Network::new(true);

    net.add_node("A");
    net.add_node("B");

    run_nonbiased_rw(&net, 0, 1, 4, "A", &["A"]);
}

#[test]
fn test_nonbiased_rw_single_walk_multi_node_connected_directed() {
    let mut net: Network<&str, f64> = Network::new(true);

    net.add_node("A");
    net.add_node("B");

    net.add_edge("A", "B", 1.0);

    run_nonbiased_rw(&net, 0, 1, 4, "A", &["A", "B"]);
}

#[test]
fn test_nonbiased_rw_single_walk_multi_node_connected_undirected() {
    let mut net: Network<&str, f64> = Network::new(false);

    net.add_node("A");
    net.add_node("B");

    net.add_edge("A", "B", 1.0);

    run_nonbiased_rw(&net, 0, 1, 4, "A", &["A", "B", "A", "B"]);
}

#[test]
fn test_nonbiased_rw_single_walk_multi_node_seed_path_directed() {
    let mut net: Network<&str, f64> = Network::new(true);

    net.add_node("A");
    net.add_node("B");
    net.add_node("C");
    net.add_node("D");

    net.add_edge("A", "B", 1.0);
    net.add_edge("B", "C", 1.0);
    net.add_edge("A", "D", 1.0);

    run_nonbiased_rw(&net, 1, 1, 3, "A", &["A", "B", "C"]);
}
