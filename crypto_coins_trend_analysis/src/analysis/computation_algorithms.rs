use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use crate::data_loader::Transaction;
use crate::utility::helper_algorithm::dijkstra;
use crate::helper_algorithm::Distance;

//compute_density(), compute_degree_distribution, all_shortest_paths, compute_centrality, 
//normalize_degree, normalize_betweenness, compute_statistics, find_top_nodes, analyze_centrality_across_periods

pub fn compute_density(graph: &HashMap<String, HashMap<String, Transaction>>) -> f64 {
    let num_nodes = graph.len();
    let num_edges: usize = graph.values().map(|edges| edges.len()).sum();
  
    if num_nodes < 2 {
        return 0.0; // A single-node or empty graph has no density
    }
  
    num_edges as f64 / (num_nodes as f64 * (num_nodes as f64 - 1.0))
  }

pub fn compute_degree_distribution(graph: &HashMap<String, HashMap<String, Transaction>>) -> HashMap<usize, usize> {
    let mut degree_count: HashMap<usize, usize> = HashMap::new();
  
    for node in graph.keys() {
        let degree = graph.get(node).map_or(0, |edges| edges.len());
        *degree_count.entry(degree).or_insert(0) += 1;
    }
  
    degree_count
}

pub fn analyze_graphs(
    graph_before: &HashMap<String, HashMap<String, Transaction>>,
    graph_during: &HashMap<String, HashMap<String, Transaction>>,
    graph_after: &HashMap<String, HashMap<String, Transaction>>,
) {
    // Compute density for each graph
    let density_before = compute_density(graph_before);
    let density_during = compute_density(graph_during);
    let density_after = compute_density(graph_after);

    println!("Graph Density (Before, During, After): {:.4}, {:.4}, {:.4}", density_before, density_during, density_after);

    // Compute degree distribution for each graph
    let degree_dist_before = compute_degree_distribution(graph_before);
    let degree_dist_during = compute_degree_distribution(graph_during);
    let degree_dist_after = compute_degree_distribution(graph_after);

    println!("\nDegree Distribution (Before): {:?}", degree_dist_before);
    println!("Degree Distribution (During): {:?}", degree_dist_during);
    println!("Degree Distribution (After): {:?}", degree_dist_after);

    // Optional: Visualize or analyze distributions further
}


