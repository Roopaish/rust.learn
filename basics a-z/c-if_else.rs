// Basic if...else statements

// Use a boolean variable
// If it is true show Hello else Goodbye
fn status(){
  let my_bool = false;

  if my_bool{
    println!("Hello");
  }else{
    println!("Goodbye");
  }
}

// Display ">5" or "<5" or "=5" based on the given number
fn check_value(a: i32){
  if a > 5{
    println!(">5");
  }else if a < 5{
    println!("<5");
  }else{
    println!("=5")
  }
}

fn main(){
  status();
  check_value(10);
  check_value(0);
  check_value(5);
}