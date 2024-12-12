use crate::utility::dijkstra;

//Test for helper_algorithms module
#[test]
fn test_dijkstra_simple() {
    let mut graph: HashMap<String, HashMap<String, Transaction>> = HashMap::new();

    // Create some nodes
    graph.entry("A".to_string()).or_insert_with(HashMap::new);
    graph.entry("B".to_string()).or_insert_with(HashMap::new);
    graph.entry("C".to_string()).or_insert_with(HashMap::new);

    // Create some edges with weights (transaction values)
    graph.get_mut("A").unwrap().insert("B".to_string(), Transaction::new(4.0, "USD".to_string(), 0));
    graph.get_mut("A").unwrap().insert("C".to_string(), Transaction::new(2.0, "USD".to_string(), 0));
    graph.get_mut("B").unwrap().insert("C".to_string(), Transaction::new(1.0, "USD".to_string(), 0));

    let shortest_paths = dijkstra(&graph, "A");

    assert_eq!(shortest_paths.get("A"), Some(&0.0));
    assert_eq!(shortest_paths.get("B"), Some(&4.0));
    assert_eq!(shortest_paths.get("C"), Some(&2.0));
}

#[test]
fn test_dijkstra_complex() {
    let mut graph: HashMap<String, HashMap<String, Transaction>> = HashMap::new();

    graph.entry("A".to_string()).or_insert_with(HashMap::new);
    graph.entry("B".to_string()).or_insert_with(HashMap::new);
    graph.entry("C".to_string()).or_insert_with(HashMap::new);
    graph.entry("D".to_string()).or_insert_with(HashMap::new);

    graph.get_mut("A").unwrap().insert("B".to_string(), Transaction::new(4.0, "USD".to_string(), 0));
    graph.get_mut("A").unwrap().insert("C".to_string(), Transaction::new(2.0, "USD".to_string(), 0));
    graph.get_mut("B").unwrap().insert("D".to_string(), Transaction::new(5.0, "USD".to_string(), 0));
    graph.get_mut("C").unwrap().insert("D".to_string(), Transaction::new(1.0, "USD".to_string(), 0));

    let shortest_paths = dijkstra(&graph, "A");

    assert_eq!(shortest_paths.get("A"), Some(&0.0));
    assert_eq!(shortest_paths.get("B"), Some(&4.0));
    assert_eq!(shortest_paths.get("C"), Some(&2.0));
    assert_eq!(shortest_paths.get("D"), Some(&3.0)); // A->C->D is the shortest path
}