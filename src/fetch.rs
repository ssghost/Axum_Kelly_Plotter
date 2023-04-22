use std::env;

use yahoo_finance::{history, Interval, Timestamped};

#[tokio::main]
pub async fn main() -> Result<(), ExitFailure>{
    let mut symbol: String = "AAPL".to_string();
    let data = history::retrieve_interval("AAPL", Interval::_6mo).await.unwrap();
    let mut symbol: String = "AAPL".to_string();
    if args.len() < 1 {
        println!("Using default symbol AAPL.");
    } else {
        symbol = args[1].clone();
    }
    for bar in &data {
      println!("Apple hit an intraday high of ${:.2} on {}.", bar.high, bar.datetime().format("%b %e %Y"));
    }
}