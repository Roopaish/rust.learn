// Derive is like a toString method
// #[derive(Debug)], like toString method
// #[derive(Debug, Clone, Copy)], data is copied over, so no ownership changes even without using & keyword


#[derive(Debug)]
enum Position {
  Manager,
  Worker
}

#[derive(Debug)] // All the members of Employee should also derive this Debug implementation
struct Employee {
  position: Position,
  work_hours: i64
}

#[derive(Debug, Clone, Copy)]
enum Position2 {
  Manager,
  Worker
}

#[derive(Debug, Clone, Copy)]
struct Employee2 {
  position: Position2,
  work_hours: i64
}

fn print_employee(emp: Employee){
  println!("{:?}", emp);
}

fn print_employee2(emp: Employee2){
  println!("{:?}", emp);
}

fn main() {
  let me = Employee {
    position: Position::Manager,
    work_hours: 6
  };

  print_employee(me);
  // print_employee(me); // This is not allowed, as me gets destroyed

  let me2 = Employee2 {
    position: Position2::Manager,
    work_hours: 4
  };

  print_employee2(me2);
  print_employee2(me2);
}