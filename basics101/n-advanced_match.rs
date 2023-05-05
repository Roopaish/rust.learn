enum Discount {
  Percent(i32),
  Flat(i32) // Can have more than one arguments too
}

struct Ticket {
  event: String,
  price: f64
}

fn main() {
  let n = 3;

  match n {
    3 => println!("three"),
    // We used _ before to ignore all other occurrences
    // We can use a variable name to address for all other data
    other => println!("{:?}", other) 
  }

  let discounts = vec![Discount::Flat(10), Discount::Flat(20), Discount::Percent(50)];

  for discount in discounts {
    match discount {
      Discount::Flat(10) => println!("Flat Rs.10 discount"),
      Discount::Flat(other) => println!("Flat Rs.{:?} discount", other),
      _ => (), // ignore Everything else i.e. Percent discount now
    }
  }


  let concerts = vec![
    Ticket {
      event: "Concert".to_owned(),
      price: 50.0,
    },
    Ticket {
      event: "Musical".to_owned(),
      price: 100.0,
    },
  ];

  for concert in concerts {
    match concert {
      Ticket {price: 50.0, event } => println!("Event at Rs.50.0 = {:?}", event),
      // Matching ticket with whatever price and ignoring(..) everything else i.e. event
      Ticket {price, ..} => println!("price = {:?}", price),
    }
  }
}