use crate::utility::helper_algorithm::dijkstra;
use crate::data_cleaning_load::data_loader::{Transaction, load_csv_convert_graph, display_graph};
use crate::connected_component::find_connected_component::{
    largest_connected_component, dfs_component_size,
};
use crate::computation_analysis::computation_algorithms::{
    compute_density, compute_degree_distribution, compute_centrality, normalize_degree,
    compute_statistics, find_top_nodes,
};
use std::collections::HashMap;
use std::collections::HashSet;


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


#[test]
fn test_transaction_new() {
    let transaction = Transaction::new(100.0, "ETH".to_string(), 1651104000);
    assert_eq!(transaction.value, 100.0);
    assert_eq!(transaction.unit, "ETH");
    assert_eq!(transaction.timestamp, 1651104000);
}


#[test]
fn test_load_csv_convert_graph() {
    use std::fs::File;
    use std::io::Write;

    // Create a mock CSV file
    let file_path = "test_data.csv";
    let mut file = File::create(file_path).expect("Failed to create test CSV file");
    writeln!(
        file,
        "field1,field2,from,to,timestamp,contract,value\n\
         1,2,addr1,addr2,1651104100,ETH,50.0\n\
         3,4,addr2,addr3,1651708900,BTC,100.0\n\
         5,6,addr3,addr4,1652400100,USDT,150.0"
    ).expect("Failed to write to test CSV file");

    // Call the function
    let result = load_csv_convert_graph(file_path);

    // Debugging output
    if let Err(e) = &result {
        println!("Error: {:?}", e);
    }

    // Assert
    assert!(result.is_ok(), "Function failed: {:?}", result);

    // Cleanup
    std::fs::remove_file(file_path).expect("Failed to delete test CSV file");
}

#[test]
fn test_display_graph() {
    let mut graph = HashMap::new();
    let mut connections = HashMap::new();
    connections.insert(
        "addr2".to_string(),
        Transaction::new(50.0, "ETH".to_string(), 1651104100),
    );
    graph.insert("addr1".to_string(), connections);

    display_graph(&graph, "Test Graph");

    // Since this function prints to the terminal, you can verify it manually
    // or redirect the output to validate it programmatically.
}

#[test]
fn test_dfs_component_size() {
    let mut graph = HashMap::new();

    let mut neighbors_a = HashMap::new();
    neighbors_a.insert(
        "B".to_string(),
        Transaction::new(10.0, "ETH".to_string(), 1651104000),
    );
    graph.insert("A".to_string(), neighbors_a);

    let mut neighbors_b = HashMap::new();
    neighbors_b.insert(
        "C".to_string(),
        Transaction::new(5.0, "BTC".to_string(), 1651105000),
    );
    graph.insert("B".to_string(), neighbors_b);

    graph.insert("C".to_string(), HashMap::new());


    let mut visited = HashSet::new();

    let size = dfs_component_size(&graph, "A", &mut visited);
    assert_eq!(size, 3); 
    // A -> B -> C form a connected component
}

#[test]
fn test_largest_connected_component() {
    let mut graph = HashMap::new();

    let mut neighbors_a = HashMap::new();
    neighbors_a.insert(
        "B".to_string(),
        Transaction::new(10.0, "ETH".to_string(), 1651104000),
    );
    graph.insert("A".to_string(), neighbors_a);
    //A -> B

    let mut neighbors_b = HashMap::new();
    neighbors_b.insert(
        "C".to_string(),
        Transaction::new(5.0, "BTC".to_string(), 1651105000),
    );
    graph.insert("B".to_string(), neighbors_b);
    //B -> C

    graph.insert("C".to_string(), HashMap::new());

    graph.insert("D".to_string(), HashMap::new());

    let largest = largest_connected_component(&graph);
    assert_eq!(largest, 3);
    // Largest component is A -> B -> C
}

#[test]
fn test_compute_density() {
    let mut graph = HashMap::new();

    graph.insert("A".to_string(), HashMap::new());

    assert_eq!(compute_density(&graph), 0.0);

    let mut edges_a = HashMap::new();
    edges_a.insert(
        "B".to_string(),
        Transaction::new(10.0, "ETH".to_string(), 1651104000),
    );
    graph.insert("A".to_string(), edges_a);
    graph.insert("B".to_string(), HashMap::new());

    //expected density = 0.5
    assert_eq!(compute_density(&graph), 0.5);
}

#[test]
fn test_compute_degree_distribution() {
    let mut graph = HashMap::new();

    let mut edges_a = HashMap::new();
    edges_a.insert(
        "B".to_string(),
        Transaction::new(10.0, "ETH".to_string(), 1651104000),
    );
    graph.insert("A".to_string(), edges_a);
    graph.insert("B".to_string(), HashMap::new()); // No outgoing edges from B

    let degree_distribution = compute_degree_distribution(&graph);

    // Expected: 1 node with degree 1, 1 node with degree 0
    assert_eq!(degree_distribution.get(&1), Some(&1));
    assert_eq!(degree_distribution.get(&0), Some(&1));
}

#[test]
fn test_compute_centrality() {
    let mut graph = HashMap::new();

    let mut edges_a = HashMap::new();
    edges_a.insert(
        "B".to_string(),
        Transaction::new(10.0, "ETH".to_string(), 1651104000),
    );
    graph.insert("A".to_string(), edges_a);

    let mut edges_b = HashMap::new();
    edges_b.insert(
        "C".to_string(),
        Transaction::new(5.0, "BTC".to_string(), 1651105000),
    );
    graph.insert("B".to_string(), edges_b);

    graph.insert("C".to_string(), HashMap::new());

    let (degree_centrality, closeness_centrality) = compute_centrality(&graph);

    // Verify degree centrality
    assert_eq!(degree_centrality.get("A"), Some(&1));
    assert_eq!(degree_centrality.get("B"), Some(&1));
    assert_eq!(degree_centrality.get("C"), Some(&0));

    let res: f64 = 0.0;
    // Verify closeness centrality (approximated)
    assert!(closeness_centrality.get("A").unwrap() > &res);
    assert!(closeness_centrality.get("B").unwrap() > &res);
    assert_eq!(closeness_centrality.get("C").unwrap(), &res);
}

#[test]
fn test_normalize_degree() {
    let mut centrality = HashMap::new();
    centrality.insert("A".to_string(), 3);
    centrality.insert("B".to_string(), 1);

    let max_possible_degree = 3;
    let normalized = normalize_degree(&centrality, max_possible_degree);

    let res: f64 = 1.0 / 3.0;
    assert_eq!(normalized.get("A"), Some(&1.0));
    assert_eq!(normalized.get("B"), Some(&res));
}

#[test]
fn test_compute_statistics() {
    let mut centrality = HashMap::new();
    centrality.insert("A".to_string(), 1.0);
    centrality.insert("B".to_string(), 2.0);
    centrality.insert("C".to_string(), 3.0);

    let (mean, variance) = compute_statistics(&centrality);

    // Expected mean: (1 + 2 + 3) / 3 = 2.0
    assert_eq!(mean, 2.0);

    // Expected variance: [(1-2)^2 + (2-2)^2 + (3-2)^2] / 3 = 2/3
    assert_eq!(variance, 2.0 / 3.0);
}

#[test]
fn test_find_top_nodes() {
    let mut centrality = HashMap::new();
    centrality.insert("A".to_string(), 0.5);
    centrality.insert("B".to_string(), 1.0);
    centrality.insert("C".to_string(), 0.8);

    let top_nodes = find_top_nodes(&centrality, 2);

    // Expected: B and C (top 2 nodes by centrality)
    assert_eq!(top_nodes.len(), 2);
    assert_eq!(top_nodes[0].0, "B");
    assert_eq!(top_nodes[1].0, "C");
}