// Enums 

enum Direction {
  Left,
  Right,
}

enum Color {
  Red,
  Yellow,
  Blue
}

fn display_color(color: Color) {
  match color {
    Color::Red => println!("Red"),
    Color::Yellow => println!("Yellow"),
    Color::Blue => println!("Blue"),
  }
}

fn main() {
  let go = Direction::Left;

  match go {
    Direction::Left => println!("Go left!"),
    Direction::Right => println!("Go Right!"),
  }

  let color = Color::Yellow;
  display_color(color);
}