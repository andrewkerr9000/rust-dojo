#![crate_name = "traits"]
use std::default::Default;

// The Default trait allows the definition of default values
// It defines a single static method default() with no parameters

// Default is one of a small number of traits that can be automatically derived

// Implement Default for MyStruct


struct MyStruct {
  x: int
}

impl Default for MyStruct {
  fn default() -> MyStruct {
    MyStruct{x: 99}
  }
}

fn main() {
  // How does the compiler know which implementation to call? Through the magic of type inference.
  let foo: MyStruct = Default::default();
  println!("{}",foo.x);
}