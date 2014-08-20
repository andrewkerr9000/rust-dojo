#![crate_name = "methods"]

// Instance method signatures start with a (mutable) reference to self
// this in Java and Javascript is like self in Rust

struct MyStruct {
  x: int
}

impl MyStruct {
  fn add(&mut self, y:int) {
    self.x += y;
  }
}

fn main() {
  let mut foo = MyStruct{x:1};
  foo.add(3);
  println!("Mutating to {}",foo.x);
}