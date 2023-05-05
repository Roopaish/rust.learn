// Vectors are used to store similar items in a list

fn main() {
  let mut numbers = vec![10, 20, 30, 40];

  // using numbers without &, will make the for loop the owner of numbers
  // So after for loop ends, numbers variable will be destroyed
  for num in &numbers{
    match num {
      30 => println!("Thirty"),
      _ => println!("{:?}", num),
    }
  }
  println!("Length {:?}" ,numbers.len()); // 4

  numbers.push(50);
  numbers.push(50);
  println!("Length {:?}" ,numbers.len()); // 6

  numbers.pop();
  println!("Length {:?}" ,numbers.len()); // 5

  println!("Item at 1 index = {:?} " ,numbers[1]); // 20

  for (i, num) in numbers.iter().enumerate() {
    println!("In position {} we have value {}", i, num);
  }

  println!("Is empty? {:?}", numbers.is_empty())
}