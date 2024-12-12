use crate::transaction_types::Transaction;
use crate::helper_algorithms::dijkstra;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Distance(f64);

impl Eq for Distance {}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

pub fn dijkstra(
    graph: &HashMap<String, HashMap<String, Transaction>>,
    start: &str,
) -> HashMap<String, f64> {
    let mut distances: HashMap<String, Distance> = HashMap::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut heap = BinaryHeap::new();

    distances.insert(start.to_string(), Distance(0.0));
    heap.push(std::cmp::Reverse((Distance(0.0), start.to_string())));

    while let Some(std::cmp::Reverse((dist, current))) = heap.pop() {
        if !visited.insert(current.clone()) {
            continue;
        }

        if let Some(neighbors) = graph.get(&current) {
            for (neighbor, transaction) in neighbors {
                let new_dist = Distance(dist.0 + transaction.value);
                let is_shorter = distances
                    .get(neighbor)
                    .map_or(true, |&current_dist| new_dist < current_dist);

                if is_shorter {
                    distances.insert(neighbor.clone(), new_dist);
                    heap.push(std::cmp::Reverse((new_dist, neighbor.clone())));
                }
            }
        }
    }

    distances.into_iter().map(|(k, v)| (k, v.0)).collect()
}

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


