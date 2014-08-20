#![crate_name = "default_implementations"]

// Trait methods can have default implementations.
// They are quite like Java 8 interfaces and Scala traits.

struct CartesianPoint {
  x:f32,
  y:f32
}

#[allow(dead_code)]
struct PolarPoint {
  angle: f32,
  length: f32
}

trait Point {
  fn distance_from_origin(&self) -> f32;
  fn inverse_square_distance(&self) -> f32 {
    1f32 / std::num::pow(self.distance_from_origin(),2)
  }
}

impl Point for CartesianPoint {
  fn distance_from_origin(&self) -> f32 {
    (std::num::pow(self.x, 2) + std::num::pow(self.y, 2)).sqrt()
  }
}

impl Point for PolarPoint {
  fn distance_from_origin(&self) -> f32 {
    self.length
  }
}

fn main() {
  let point1 = CartesianPoint {x: 3f32, y: 4f32};
  let point2 = PolarPoint {angle: 0f32, length: 8f32};
  println!("Inverse square distance for point1 is {}", point1.inverse_square_distance());
  println!("Inverse square distance for point2 is {}", point2.inverse_square_distance());
}