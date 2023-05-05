/*
enum Option<T> {
  Some(T),
  None
}
 */

struct Customer {
  age: Option<i32>,
  email: String,
}

fn find_customer_age(email: &str, customers: &Vec<Customer>) -> Option<i32> {
  for customer in customers {
    if customer.email == email {
      return customer.age; // Early return
    }
  }
  None // return None if not found
}

fn main() {
  let customers = vec![
    Customer {
      age: Some(20), email: "mark@gmail.com".to_owned(),
    },
    Customer {
      age: None, email: "becky@gmail.com".to_owned(),
    }
  ];

  for customer in &customers {
    match customer.age {
      Some(age) => println!("User {:?} age is {:?}", customer.email, age),
      None => println!("Use {:?} did not provide his/her name", customer.email),
    }
  }

  let age = find_customer_age("mark@gmail.com" ,&customers);
  println!("Age is {:?}", age);
}