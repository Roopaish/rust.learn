fn current_coordinates() -> (i32, i32) {
  (10, 90)
} 

fn main() {
  let coord = (2, 3);
  println!("{:?}, {:?}", coord.0, coord.1);

  // Destructuring tuples
  let (x, y) = (2, 3);
  println!("{:?}, {:?}", x, y);

  let (name, age) = ("User", 20);

  let (x, y) = current_coordinates();
  println!("Current coordinate: {:?}, {:?}", x, y);
}