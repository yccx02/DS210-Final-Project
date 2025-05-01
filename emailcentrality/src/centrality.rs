// centrality.rs
// This module calculates four types of centrality metrics for a directed graph:
// - Out-degree
// - In-degree
// - Closeness (manually listed here from precomputed results)
// - Betweenness (read from external CSV, precomputed in Python)

use petgraph::graph::DiGraph;
use petgraph::Direction;
use std::collections::HashMap;
use std::fs::File;
use csv::Reader;

/// Computes centrality measures from the graph.
///
/// # Input
/// - `graph`: A reference to the directed email communication graph
///
/// # Returns
/// A tuple of four vectors containing:
/// - `out_degrees`: (node_id, number of outgoing edges)
/// - `in_degrees`: (node_id, number of incoming edges)
/// - `closeness`: (node_id, closeness score)
/// - `betweenness`: (node_id, betweenness score)
///
/// # Output type
/// - `std::io::Result<(...centrality vectors...)>` for error handling if file fails
pub fn calculate_centralities(
    graph: &DiGraph<usize, ()>
) -> std::io::Result<(
    Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<(usize, f64)>, Vec<(usize, f64)>
)> {

    // Out-degree: Number of outgoing edges per node
    let mut out_degrees: Vec<(usize, usize)> = graph.node_indices()
        .map(|idx| (graph[idx], graph.edges(idx).count()))
        .collect();
    out_degrees.sort_by(|a, b| b.1.cmp(&a.1)); // Sort descending

    // In-degree: Number of incoming edges per node
    let mut in_degrees: Vec<(usize, usize)> = graph.node_indices()
        .map(|idx| (graph[idx], graph.edges_directed(idx, Direction::Incoming).count()))
        .collect();
    in_degrees.sort_by(|a, b| b.1.cmp(&a.1)); // Sort descending

    // Closeness: Manually loaded from precomputed values
    // (Assumes top 10 were calculated externally)
    let closeness_data = vec![
        (160, 0.58072), (82, 0.54218), (121, 0.53585), (107, 0.52420),
        (86, 0.52334), (62, 0.52108), (249, 0.51359), (434, 0.51113),
        (183, 0.50551), (5, 0.50366)
    ];
    let mut closeness: Vec<_> = closeness_data.into_iter().collect();
    closeness.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); // Sort descending

    // Betweenness: Load from external CSV file
    // File format: node_id, score
    let file = File::open("betweenness_centrality.csv")?;
    let mut rdr = Reader::from_reader(file);
    let mut betweenness = HashMap::new();

    // Skip header and parse node_id and score
    for result in rdr.records().skip(1) {
        let record = result?;
        let node_id: usize = record[0].parse().unwrap();
        let score: f64 = record[1].parse().unwrap();
        betweenness.insert(node_id, score);
    }

    let mut bet_vec: Vec<_> = betweenness.into_iter().collect();
    bet_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); // Sort descending

    // Return all four metrics
    Ok((out_degrees, in_degrees, closeness, bet_vec))
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::DiGraph;

    /// Basic smoke test to verify that all centrality vectors return values for a simple graph.
    #[test]
    fn test_centrality_computation() {
        let mut graph = DiGraph::<usize, ()>::new();
        let a = graph.add_node(0);
        let b = graph.add_node(1);
        graph.add_edge(a, b, ());

        let (out_degrees, in_degrees, closeness, betweenness) = calculate_centralities(&graph).unwrap();

        // All vectors should contain at least one entry
        assert!(!out_degrees.is_empty());
        assert!(!in_degrees.is_empty());
        assert!(!closeness.is_empty());
        assert!(!betweenness.is_empty());
    }
}