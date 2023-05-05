// Struct -> use to create data structures

struct GroceryItem {
  stock: i32,
  price: f64,
}

enum Flavor {
  Sparkling,
  Sweet,
  Minty,
  Fruity
}

struct Drink {
  flavor: Flavor,
  quantity: f64,
}

fn display_drink(drink: Drink){
  match drink.flavor {
    Flavor::Sparkling => println!("Sparkling"),
    Flavor::Sweet => println!("Sweet"),
    Flavor::Minty => println!("Minty"),
    Flavor::Fruity => println!("Fruity")
  }

  println!("quantity = {:?}", drink.quantity)
}

fn main() {
  let cereal = GroceryItem{stock: 10, price: 100.0};
  println!("Stock = {:?}, Price = {:?}", cereal.stock, cereal.price);

  let drink = Drink{flavor: Flavor::Fruity, quantity: 100.0};
  display_drink(drink);
}