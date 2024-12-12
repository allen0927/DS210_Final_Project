use std::collections::HashMap;
use std::collections::HashSet;
use crate::data_loader::Transaction;

pub fn largest_connected_component(graph: &HashMap<String, HashMap<String, Transaction>>) -> usize {
  let mut visited = HashSet::new();
  let mut max_size = 0;

  for node in graph.keys() {
      if !visited.contains(node) {
          let size = dfs_component_size(graph, node, &mut visited);
          max_size = max_size.max(size);
      }
  }

  max_size
}

fn dfs_component_size(
  graph: &HashMap<String, HashMap<String, Transaction>>,
  start: &str,
  visited: &mut HashSet<String>,
) -> usize {
  let mut stack = vec![start.to_string()];
  let mut size = 0;

  while let Some(node) = stack.pop() {
      if visited.insert(node.clone()) {
          size += 1;
          if let Some(neighbors) = graph.get(&node) {
              for neighbor in neighbors.keys() {
                  if !visited.contains(neighbor) {
                      stack.push(neighbor.clone());
                  }
              }
          }
      }
  }

  size
}

pub fn analyze_largest_components(
  graph_before: &HashMap<String, HashMap<String, Transaction>>,
  graph_during: &HashMap<String, HashMap<String, Transaction>>,
  graph_after: &HashMap<String, HashMap<String, Transaction>>,
) {
  // Find the largest connected component for each graph
  let largest_before = largest_connected_component(graph_before);
  let largest_during = largest_connected_component(graph_during);
  let largest_after = largest_connected_component(graph_after);

  // Output the results
  println!("Largest Connected Component:");
  println!("Before Crash: {}", largest_before);
  println!("During Crash: {}", largest_during);
  println!("After Crash: {}", largest_after);
}