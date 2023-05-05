// Hashmap: Data is located using a key (key-value pair)
// should be mutable, cause data has to be entered manually

use std::collection::Hashmap;

fn main() {
  let mut stocks = Hashmap::new();
  stocks.insert("Chair", 5);
  stocks.insert("Table", 10);
  stocks.insert("TV", 2);

  let mut total = 0;
  for (item, qty) in stocks.iter() {
    let stock_count = if qty == &0 {
      "out of stock".to_owned(),
    } else {
      format!("{:?}", qty), // create a string using int
    }

    println!("item = {:?}, stock = {:?}", item, stock_count);
    total += 1;
  }

  println!("Total items = {:?}", total);
}