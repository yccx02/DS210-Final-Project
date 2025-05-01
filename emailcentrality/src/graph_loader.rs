// graph_loader.rs
// This module loads the graph structure and department labels from the SNAP
// email-Eu-core dataset. It returns a directed graph and a label map.

use petgraph::graph::DiGraph;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Loads the email communication graph and department labels from files.
///
/// # Returns
/// - `DiGraph<usize, ()>`: Directed graph of emails (node ID = usize)
/// - `HashMap<usize, usize>`: Maps node ID to department ID
///
/// # Errors
/// Returns `std::io::Result` in case of file I/O errors or bad formatting.
pub fn load_graph_and_labels() -> std::io::Result<(DiGraph<usize, ()>, HashMap<usize, usize>)> {
    // Load edges from email-Eu-core.txt
    // Format: <src> <dst> (each line is a directed edge)
    let edge_file = File::open("email-Eu-core.txt")?;
    let reader = BufReader::new(edge_file);

    let mut node_map = HashMap::new(); // Map node ID to NodeIndex
    let mut graph = DiGraph::<usize, ()>::new(); // Directed graph with node weights = usize

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() != 2 { continue; }

        // Parse source and destination node IDs
        let src: usize = parts[0].parse().unwrap();
        let dst: usize = parts[1].parse().unwrap();

        // Add nodes to graph if not already present
        let src_idx = *node_map.entry(src).or_insert_with(|| graph.add_node(src));
        let dst_idx = *node_map.entry(dst).or_insert_with(|| graph.add_node(dst));

        // Add a directed edge
        graph.add_edge(src_idx, dst_idx, ());
    }

    // Load labels from email-Eu-core-department-labels.txt
    // Format: <node_id> <department_id>
    let label_file = File::open("email-Eu-core-department-labels.txt")?;
    let label_reader = BufReader::new(label_file);

    let mut labels = HashMap::new();
    for line in label_reader.lines() {
        let line = line?;
        let parts: Vec<_> = line.trim().split_whitespace().collect();
        if parts.len() == 2 {
            if let (Ok(node_id), Ok(dept_id)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                labels.insert(node_id, dept_id);
            }
        }
    }

    // Return both the graph and department label map
    Ok((graph, labels))
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// Loads the full dataset and checks that graph and labels are non-empty.
    #[test]
    fn test_graph_loading() {
        let (graph, labels) = load_graph_and_labels().expect("Failed to load graph or labels");
        assert!(graph.node_count() > 0, "Graph should contain nodes");
        assert!(labels.len() > 0, "Labels should be loaded");
    }
}

