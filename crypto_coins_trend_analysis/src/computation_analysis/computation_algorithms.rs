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

}


pub fn compute_centrality(
    graph: &HashMap<String, HashMap<String, Transaction>>,
  ) -> (HashMap<String, usize>, HashMap<String, f64>) {
    let mut degree_centrality: HashMap<String, usize> = HashMap::new();
    let mut closeness_centrality: HashMap<String, f64> = HashMap::new();
  
    // Degree Centrality
    for node in graph.keys() {
        let degree = graph.get(node).map_or(0, |edges| edges.len());
        degree_centrality.insert(node.clone(), degree);
    }

    println!("finished computation of one graph's degree centrality......");
  
    // Closeness Centrality
    for node in graph.keys() {
        let shortest_paths = dijkstra(graph, node);
        let total_distance: f64 = shortest_paths.values().sum();
        let closeness = if total_distance > 0.0 {
            (graph.len() - 1) as f64 / total_distance
        } else {
            0.0
        };
        closeness_centrality.insert(node.clone(), closeness);
    }
    
    println!("finished computation of one graph's closeness centrality......");
    (degree_centrality, closeness_centrality)
}

pub fn all_shortest_paths(
    graph: &HashMap<String, HashMap<String, Transaction>>,
    start: &str,
) -> HashMap<String, Vec<Vec<String>>> {
    let mut shortest_paths: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    let mut distances: HashMap<String, Distance> = HashMap::new();
    let mut heap = BinaryHeap::new();

    distances.insert(start.to_string(), Distance(0.0));
    heap.push(std::cmp::Reverse((Distance(0.0), start.to_string())));

    while let Some(std::cmp::Reverse((dist, current))) = heap.pop() {
        if dist > *distances.get(&current).unwrap_or(&Distance(f64::INFINITY)) {
            continue;
        }

        if !shortest_paths.contains_key(&current) {
            shortest_paths.insert(current.clone(), vec![vec![start.to_string()]]);
        }

        if let Some(neighbors) = graph.get(&current) {
            for (neighbor, transaction) in neighbors {
                let new_dist = Distance(dist.0 + transaction.value);

                if new_dist < *distances.get(neighbor).unwrap_or(&Distance(f64::INFINITY)) {
                    distances.insert(neighbor.clone(), new_dist);

                    let current_paths = shortest_paths.get(&current).unwrap().clone(); // Store the current paths
                    shortest_paths.insert(neighbor.clone(), current_paths);

                    if let Some(paths) = shortest_paths.get_mut(neighbor) {
                        for path in paths.iter_mut() {
                            path.push(neighbor.clone());
                        }
                    }

                    heap.push(std::cmp::Reverse((new_dist, neighbor.clone())));
                } else if new_dist == *distances.get(neighbor).unwrap_or(&Distance(f64::INFINITY)) {
                    let current_paths = shortest_paths.get(&current).unwrap().clone(); // Store the current paths

                    if let Some(paths) = shortest_paths.get_mut(neighbor) {
                        for mut path in current_paths {
                            path.push(neighbor.clone());
                            paths.push(path);
                        }
                    }
                }
            }
        }
    }
    shortest_paths
}

pub fn compute_betweeness_centrality(
    graph: &HashMap<String, HashMap<String, Transaction>>
) -> HashMap<String, f64>{
    //betweeness centrality
    let mut betweenness_centrality: HashMap<String, f64> = HashMap::new();

    for source in graph.keys() {
        let all_paths_from_source = all_shortest_paths(graph, source);

        for target in graph.keys() {
            if source == target {
                continue;
            }

            if let Some(paths) = all_paths_from_source.get(target) {
                for path in paths {
                    for node in path.iter().skip(1).take(path.len() - 2) { // Exclude source and target
                        *betweenness_centrality.entry(node.clone()).or_insert(0.0) += 1.0 / paths.len() as f64;
                    }
                }
            }
        }
    }

    // Normalize betweenness centrality
    let n = graph.len() as f64;
    for centrality in betweenness_centrality.values_mut() {
        *centrality /= (n - 1.0) * (n - 2.0);
    }
    
    betweenness_centrality
}
fn normalize_degree(centrality: &HashMap<String, usize>, max_possible_degree: usize) -> HashMap<String, f64> {
    centrality.iter()
        .map(|(node, &degree)| (node.clone(), degree as f64 / max_possible_degree as f64))
        .collect()
}

fn normalize_betweenness(centrality: &HashMap<String, f64>, n: usize) -> HashMap<String, f64> {
    let max_possible = (n - 1) as f64 * (n - 2) as f64;
    centrality.iter()
        .map(|(node, &value)| (node.clone(), value / max_possible))
        .collect()
}

fn compute_statistics(centrality: &HashMap<String, f64>) -> (f64, f64) {
    let n = centrality.len() as f64;
    let mean = centrality.values().sum::<f64>() / n;
    let variance = centrality.values().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
    (mean, variance)
}

fn find_top_nodes(centrality: &HashMap<String, f64>, top_n: usize) -> Vec<(String, f64)> {
    let mut centrality_vec: Vec<_> = centrality.iter().collect();
    centrality_vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    centrality_vec.into_iter().take(top_n).map(|(k, v)| (k.clone(), *v)).collect()
}

pub fn analyze_centrality_across_periods(
  graph_before: &HashMap<String, HashMap<String, Transaction>>,
  graph_during: &HashMap<String, HashMap<String, Transaction>>,
  graph_after: &HashMap<String, HashMap<String, Transaction>>,
) {
    // Compute centralities for each period
    let (degree_before, closeness_before) = compute_centrality(graph_before);
    let (degree_during, closeness_during) = compute_centrality(graph_during);
    let (degree_after, closeness_after) = compute_centrality(graph_after);

    // Normalize centrality metrics
    let norm_degree_before = normalize_degree(&degree_before, graph_before.len() - 1);
    let norm_degree_during = normalize_degree(&degree_during, graph_during.len() - 1);
    let norm_degree_after = normalize_degree(&degree_after, graph_after.len() - 1);


    // Compute summary statistics
    let (mean_degree_before, var_degree_before) = compute_statistics(&norm_degree_before);
    let (mean_degree_during, var_degree_during) = compute_statistics(&norm_degree_during);
    let (mean_degree_after, var_degree_after) = compute_statistics(&norm_degree_after);
    println!("=== Centrality Analysis ===");
    println!("Degree Centrality Mean (Before, During, After): {:.2}, {:.2}, {:.2}", mean_degree_before, mean_degree_during, mean_degree_after);
    println!("Degree Centrality Variance (Before, During, After): {:.2}, {:.2}, {:.2}", var_degree_before, var_degree_during, var_degree_after);

    // Identify top nodes
    let top_degree_before = find_top_nodes(&norm_degree_before, 5);
    let top_degree_during = find_top_nodes(&norm_degree_during, 5);
    let top_degree_after = find_top_nodes(&norm_degree_after, 5);
    println!("\nTop Degree Nodes (Before): {:?}", top_degree_before);
    println!("Top Degree Nodes (During): {:?}", top_degree_during);
    println!("Top Degree Nodes (After): {:?}", top_degree_after);

/*
  let betweenness_before = compute_betweeness_centrality(graph_before);
  let betweenness_during = compute_betweeness_centrality(graph_during);
  let betweenness_after = compute_betweeness_centrality(graph_after);

  let norm_betweenness_before = normalize_betweenness(&betweenness_before, graph_before.len());
  let norm_betweenness_during = normalize_betweenness(&betweenness_during, graph_during.len());
  let norm_betweenness_after = normalize_betweenness(&betweenness_after, graph_after.len());


  let (mean_betweenness_before, var_betweenness_before) = compute_statistics(&norm_betweenness_before);
  let (mean_betweenness_during, var_betweenness_during) = compute_statistics(&norm_betweenness_during);
  let (mean_betweenness_after, var_betweenness_after) = compute_statistics(&norm_betweenness_after);
  println!("Betweenness Centrality Mean (Before, During, After): {:.2}, {:.2}, {:.2}", mean_betweenness_before, mean_betweenness_during, mean_betweenness_after);
  println!("Betweenness Centrality Variance (Before, During, After): {:.2}, {:.2}, {:.2}", var_betweenness_before, var_betweenness_during, var_betweenness_after);

  let top_betweenness_before = find_top_nodes(&norm_betweenness_before, 5);
  let top_betweenness_during = find_top_nodes(&norm_betweenness_during, 5);
  let top_betweenness_after = find_top_nodes(&norm_betweenness_after, 5);

  println!("\nTop Betweenness Nodes (Before): {:?}", top_betweenness_before);
  println!("Top Betweenness Nodes (During): {:?}", top_betweenness_during);
  println!("Top Betweenness Nodes (After): {:?}", top_betweenness_after);
*/
}
