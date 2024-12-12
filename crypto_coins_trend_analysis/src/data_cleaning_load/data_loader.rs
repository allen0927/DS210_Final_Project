use std::collections::HashMap;
use csv::ReaderBuilder;

//Transaction, load_csv_convert_graph, display_graph
#[derive(Clone)] 
pub struct Transaction {
  pub value: f64,           //value of transaction
  pub unit: String,         //coin used
  pub timestamp: u64,       //time of transaction, if time1 < time2, then time1 is earlier than time2
}

impl Transaction {
  pub fn new(value: f64, unit:String, timestamp: u64) -> Self {
      Transaction {
          value,
          unit,
          timestamp,
      }
  }
}

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

  for result in rdr.records() {
      let record = result?;

      let from_address: String = record[2].to_string();
      let to_address: String = record[3].to_string();
      let timestamp: u64 = record[4].parse::<u64>()?;
      let contract_address: String = record[5].to_string();
      let value: f64 = record[6].parse::<f64>()?;

      let transaction = Transaction::new(value, contract_address, timestamp);

      if timestamp >= start_time && timestamp < during_crash_time {
          graph_prior_crash
              .entry(from_address.clone())
              .or_insert_with(HashMap::new)
              .insert(to_address.clone(), transaction.clone());
      } else if timestamp >= during_crash_time && timestamp < after_crash_time {
          graph_during_crash
              .entry(from_address.clone())
              .or_insert_with(HashMap::new)
              .insert(to_address.clone(), transaction.clone());
      } else if timestamp >= after_crash_time && timestamp < end_time {
          graph_after_crash
              .entry(from_address.clone())
              .or_insert_with(HashMap::new)
              .insert(to_address.clone(), transaction.clone());
      }
  }
  println!("finished data loading......");
  Ok((graph_prior_crash, graph_during_crash, graph_after_crash))
}


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