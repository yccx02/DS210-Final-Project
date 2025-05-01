// main.rs; import module
mod graph_loader;
mod centrality;
mod analysis;

use petgraph::graph::DiGraph;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    // Load the graph and labels
    let (graph, labels) = graph_loader::load_graph_and_labels()?;

    println!("Graph loaded: {} nodes, {} edges.", graph.node_count(), graph.edge_count());
    println!("Labels loaded: {} nodes with departments.", labels.len());

    // Calculate centralities
    let (out_degrees, in_degrees, closeness, betweenness) = centrality::calculate_centralities(&graph)?;

    // Analyze and display results
    analysis::analyze_and_display(&graph, &labels, &out_degrees, &in_degrees, &closeness, &betweenness);

    Ok(())
}
