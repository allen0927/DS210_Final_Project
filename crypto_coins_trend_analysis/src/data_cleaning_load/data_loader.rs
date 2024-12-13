use std::collections::HashMap;
use csv::ReaderBuilder;

//Transaction, load_csv_convert_graph, display_graph

/**************************************************************
*
*   The datastructure defined to represent the weight of the graph
*   which includes three fields:
*   value: represent the numerical value used in transaction
*   unit: the coin used in this transaction, also unit of value
*   timestamp: the time of transaction executed, express in unix epoch time
*
***************************************************************/

#[derive(Clone)] 
pub struct Transaction {
  pub value: f64,           //value of transaction
  pub unit: String,         //coin used
  pub timestamp: u64,       //time of transaction, if time1 < time2, then time1 is earlier than time2
}

/**************************************************************
*
*   The constructor of a Transaction type instances
*
***************************************************************/

impl Transaction {
  pub fn new(value: f64, unit:String, timestamp: u64) -> Self {
      Transaction {
          value,
          unit,
          timestamp,
      }
  }
}

/**************************************************************
*
*   The function that load the dataset and convert into weighted graph
*   the weighted graph is defined as:
*   HashMap<String, HashMap<String, Transaction>>
*   The key(node) in hashmap is the from_address in dataset, which represented
*   in String type, the value of outer hashmap represent the outdegree of
*   of current key(node), the inner hashmap the store the weight and node that
*   outer key pointed to.
*
***************************************************************/

pub fn load_csv_convert_graph(
  path: &str,
) -> Result<(HashMap<String, HashMap<String, Transaction>>,
           HashMap<String, HashMap<String, Transaction>>,
           HashMap<String, HashMap<String, Transaction>>),
          Box<dyn std::error::Error>> 
{
  let mut rdr = ReaderBuilder::new().from_path(path)?;

  let start_time: u64 = 1651104000;       // Start time
  let during_crash_time: u64 = 1651708800; // During crash start
  let after_crash_time: u64 = 1652400000; // After crash start
  let end_time: u64 = 1652918400;         // End time

  let mut graph_prior_crash: HashMap<String, HashMap<String, Transaction>> = HashMap::new();
  let mut graph_during_crash: HashMap<String, HashMap<String, Transaction>> = HashMap::new();
  let mut graph_after_crash: HashMap<String, HashMap<String, Transaction>> = HashMap::new();

  let mut count_before = 0;
  let mut count_during = 0;
  let mut count_after = 0;
  for result in rdr.records() {
      let record = result?;

      let from_address: String = record[2].to_string();
      let to_address: String = record[3].to_string();
      let timestamp: u64 = record[4].parse::<u64>()?;
      let contract_address: String = record[5].to_string();
      let value: f64 = record[6].parse::<f64>()?;

      let transaction = Transaction::new(value, contract_address, timestamp);

      if timestamp >= start_time && timestamp < during_crash_time && count_before < 10000 {
          graph_prior_crash
              .entry(from_address.clone())
              .or_insert_with(HashMap::new)
              .insert(to_address.clone(), transaction.clone());
          count_before += 1;
      }
      else if timestamp >= during_crash_time && timestamp < after_crash_time && count_during < 10000 {
          graph_during_crash
              .entry(from_address.clone())
              .or_insert_with(HashMap::new)
              .insert(to_address.clone(), transaction.clone());
          count_during += 1;
      }
      else if timestamp >= after_crash_time && timestamp < end_time && count_after < 10000 {
          graph_after_crash
              .entry(from_address.clone())
              .or_insert_with(HashMap::new)
              .insert(to_address.clone(), transaction.clone());
          count_after += 1;
      }
  }
  println!("finished data loading......");
  Ok((graph_prior_crash, graph_during_crash, graph_after_crash))
}

/**************************************************************
*
*   The helper function to display the defined graph in a more
*   human-readable way
*
***************************************************************/

pub fn display_graph(graph: &HashMap<String, HashMap<String, Transaction>>, graph_name: &str) {
  println!("Graph: {}", graph_name);
  for (from_address, connections) in graph {
      println!("  From: {}", from_address);
      for (to_address, transaction) in connections {
          println!(
              "    To: {}, Value: {}, Unit: {}, Timestamp: {}",
              to_address, transaction.value, transaction.unit, transaction.timestamp
          );
      }
  }
  println!();
}