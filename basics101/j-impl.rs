// impl (implementation block) to define any methods for struct

struct Temperature {
  degrees_f: f64,
}

impl Temperature {
  // Self = Temperature, Can be use explicitly too
  fn freezing() -> Self {
    Self {
      degrees_f: 32.0
    }
  }

  fn show_temp(&self) {
    println!("{:?} degrees F", self.degrees_f);
  }
}

fn main() {
  let hot = Temperature {degrees_f: 100.0};
  hot.show_temp();

  let freezing = Temperature::freezing();
  freezing.show_temp();
}