#![allow(unused)]
fn main() {
 let v = vec![1, 2];

  struct Point {
    x: i32,
    y: i32,
  }

  use std::fmt;

  impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "({}, {})", self.x, self.y)
    }
  }

  trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
      let output = self.to_string();
      let len = output.len();
      println!("{}", "*".repeat(len + 4));
      println!("*{}*", " ".repeat(len + 2));
      println!("* {} *", output);
      println!("*{}*", " ".repeat(len + 2));
      println!("{}", "*".repeat(len + 4));
    }
  }

  impl OutlinePrint for Point {}

  Point { x: 2, y: 7 }.outline_print();
}
