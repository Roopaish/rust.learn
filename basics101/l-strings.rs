// Two types
// String -> owned
// &str -> borrowed String slice
// Must use owned String while using struct

struct Person {
  age: i32,
  name: String,
  fav_color: String,
}

fn display_name(name: &str) {
  println!("Name = {:?}", name);
}

fn display_fav_color(color: &str) {
  println!("Fav color = {:?}", color);
}

fn main() {
  let persons = vec![
    Person {
      age: 32,
      name: "John".to_owned(),
      fav_color: "red".to_owned(),
    },
    Person {
      age: 22,
      name: String::from("Roopaish"),
      fav_color: String::from("Paper"),
    },
    Person {
      age: 9,
      name: String::from("Sanjay"),
      fav_color: String::from("Purple"),
    }
  ];

  for person in persons {
    if person.age > 10 {
    display_name(&person.name);
    display_fav_color(&person.fav_color);
    }
  }

  let name = "John Snow";
  println!("{:?} to uppercase: {:?}", name, name.to_uppercase());
  println!("{:?} to uppercase: {:?}", name, name.to_lowercase());
}