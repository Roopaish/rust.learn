// Match are Like select-case statements

// Using match keyword to determine which message to display
fn simple_macro() {
  let my_bool = false;

  match my_bool {
    true => println!("Boolean is true"),
    false => println!("Boolean is false"),
  }
}

fn get_number_in_word(a:i32) {
  match a{
    1 => println!("One"),
    2 => println!("Two"),
    3 => println!("Three"),
    4 => println!("Four"),
    _ => println!("Not One, Two, Three or Four 😂"),
  }
}

fn main() {
  simple_macro();
  get_number_in_word(4);
  get_number_in_word(9);
}