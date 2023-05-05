// Basic Arithmetic
// One fxn to add two numbers
// One fxn to display the result
// Use "{:?}" token in println macro to display the result

fn add(a: i32, b: i32) -> i32 {
  a+b
}

fn display_result(result: i32){
  println!("{:?}", result);
}

fn main(){
  let result = add(2, 2);
  display_result(result);
}