#![crate_name = "inheritance"]

// Traits can inherit from other traits.
// Structs do not inherit from other structs.
// There is multiple inheritance of behaviour and interfaces
// There is no inheritance of state

trait Shape {
  fn area(&self) -> f32;
  fn circumference(&self) -> f32;
}

trait Polygon {
  fn sides(&self) -> int;
}

struct Square {
  width: f32,
  height: f32
}

impl Polygon for Square {
  fn sides(&self) -> int {
    4
  }
}

impl Shape for Square {
  fn area(&self) -> f32 {
    self.width * self.height
  }
}

fn main() {
  let myShape = Square{width: 3f32, height: 4f32};
  println!("My shape has area of {} and {} sides", myShape.area(), myShape.sides());
}