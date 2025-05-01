// analysis.rs
// This module analyzes and compares the results from multiple centrality measures.
// It prints top nodes per metric, detects overlaps, and outputs summary results.

use petgraph::graph::DiGraph;
use std::collections::{HashMap, HashSet};

/// Analyzes and displays results from multiple centrality measures.
///
/// # Inputs
/// - `graph`: The communication graph
/// - `labels`: Map of node ID → department ID
/// - `out_degrees`, `in_degrees`: Vectors of degree centrality (node ID, count)
/// - `closeness`, `betweenness`: Vectors of centrality scores (node ID, score)
///
/// # Behavior
/// - Prints top 10 nodes per centrality measure
/// - Identifies nodes that appear in all top 10s
/// - Summarizes overlap across metrics
pub fn analyze_and_display(
    graph: &DiGraph<usize, ()>,
    labels: &HashMap<usize, usize>,
    out_degrees: &Vec<(usize, usize)>,
    in_degrees: &Vec<(usize, usize)>,
    closeness: &Vec<(usize, f64)>,
    betweenness: &Vec<(usize, f64)>
) {

    // Step 1: Print top 10 rankings
    println!("Top 10 nodes by out-degree (emails sent):");
    for (i, (node_id, degree)) in out_degrees.iter().take(10).enumerate() {
        let dept = labels.get(node_id).unwrap_or(&9999);
        println!("{}. Node {} (Dept {}): {} emails sent", i + 1, node_id, dept, degree);
    }

    println!("\nTop 10 nodes by in-degree (emails received):");
    for (i, (node_id, degree)) in in_degrees.iter().take(10).enumerate() {
        let dept = labels.get(node_id).unwrap_or(&9999);
        println!("{}. Node {} (Dept {}): {} emails received", i + 1, node_id, dept, degree);
    }

    println!("\nTop 10 nodes by closeness centrality:");
    for (i, (node_id, score)) in closeness.iter().take(10).enumerate() {
        let dept = labels.get(node_id).unwrap_or(&9999);
        println!("{}. Node {} (Dept {}): {:.5}", i + 1, node_id, dept, score);
    }

    println!("\nTop 10 nodes by betweenness centrality:");
    for (i, (node_id, score)) in betweenness.iter().take(10).enumerate() {
        let dept = labels.get(node_id).unwrap_or(&9999);
        println!("{}. Node {} (Dept {}): {:.5}", i + 1, node_id, dept, score);
    }

    // Step 2: Build top-10 node sets for each measurement
    let top_out: HashSet<_> = out_degrees.iter().take(10).map(|(node_id, _)| *node_id).collect();
    let top_in: HashSet<_> = in_degrees.iter().take(10).map(|(node_id, _)| *node_id).collect();
    let top_closeness: HashSet<_> = closeness.iter().take(10).map(|(node_id, _)| *node_id).collect();
    let top_bet: HashSet<_> = betweenness.iter().take(10).map(|(node_id, _)| *node_id).collect();

    // Step 3: Find intersection of all four top 10 sets
    let all_top = top_out
        .intersection(&top_in)
        .cloned()
        .collect::<HashSet<_>>()
        .intersection(&top_closeness)
        .cloned()
        .collect::<HashSet<_>>()
        .intersection(&top_bet)
        .cloned()
        .collect::<HashSet<_>>();

    // Output overlapping nodes
    println!("\nNodes in the top 10 of ALL four centrality measures:");
    for node_id in &all_top {
        let dept = labels.get(node_id).unwrap_or(&9999);
        println!("Node {} (Dept {})", node_id, dept);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::DiGraph;

    /// test for `analyze_and_display()` to ensure no panics with small graphs.
    #[test]
    fn test_analysis_output() {
        // Create a small test graph
        let mut graph = DiGraph::<usize, ()>::new();
        let a = graph.add_node(0);
        let b = graph.add_node(1);
        graph.add_edge(a, b, ());

        let mut labels = std::collections::HashMap::new();
        labels.insert(0, 1);
        labels.insert(1, 2);

        let out = vec![(0, 1)];
        let in_deg = vec![(1, 1)];
        let closeness = vec![(0, 0.5)];
        let betweenness = vec![(0, 0.1)];

        analyze_and_display(&graph, &labels, &out, &in_deg, &closeness, &betweenness);
        // We don't assert output, just verify it doesn't panic
    }
}
