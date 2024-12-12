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