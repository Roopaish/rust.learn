// Result is a data type that contains one of two types of data
// Successful data or Error data

/*
enum Error<T, E>{
  Ok(T),
  Err(E)
}
 */
`
struct Adult {
  age: u8,
  name: String,
}

impl Adult {
  fn new (age: u8, name: &str) -> Result<Self, &str> {
    if age > 21 {
      Ok(Self{ age, name: name.to_string(), })
    }else {
      Err("Age must be 21")
    }
  }
}

fn main() {
  let child = Adult::new(15, "Romeo");
  let person = Adult::new(22, "Sanjay");

  match child {
    Ok(c) => println!("{} is {} years old.", c.name, c.age),
    Err(e) => println!("Error: {}",e),
  }

  match person {
    Ok(p) => println!("{} is {} years old.", p.name, p.age),
    Err(e) => println!("Error: {}",e),
  }
}