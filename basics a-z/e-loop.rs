// Looping using loop statement

fn main() {
  let mut n = 1;

  println!("Using loop statement");
  loop {
    println!("{:?}", n);
    if n == 4{
      break;
    }
    n += 1;
  }

  n = 1;
  println!("Using while statements");
  while n <= 3 {
    println!("{:?}", n);
    n += 1;
  }
}