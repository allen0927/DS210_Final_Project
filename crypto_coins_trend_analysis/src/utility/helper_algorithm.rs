use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use crate::data_loader::Transaction;

//dijkstra
/**************************************************************
*
*   The datastructure Distance represent teh distance of graph,
*   which also represent the weight value, but since weight is
*   f64, could not directly used in Binary Heap, so defined a
*   new datastructure that implements tha required traits for
*   Binary Heap
*
***************************************************************/
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Distance(pub f64);

impl Eq for Distance {}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

/**************************************************************
*
*   The helper function dijkstra algorithm for calculation of
*   shortest path in the weighted graph(non-negative weight),
*   implemented to fit the defined graph in this project
*
***************************************************************/

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