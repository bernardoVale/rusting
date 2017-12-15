struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
  fn print_description(&self) {
      println!("Rectangle: {} x {}", self.width, self.height);
  }
}

// You can implement multiple times
impl Rectangle {
  fn is_square(&self) -> bool {
      self.height == self.width // implicitly return statement
  }
}
fn main() {
    let my_rect = Rectangle { width: 10, height: 5 };

    my_rect.print_description();
    println!("Is square? {}", my_rect.is_square());
}