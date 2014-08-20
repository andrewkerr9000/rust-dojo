#![crate_name = "dynamic"]
use std::iter::AdditiveIterator;

struct MyFirstStruct {
  x: int
}

struct MySecondStruct {
  y: int
}

trait MyTrait {
  fn value(&self) -> int;
}

impl MyTrait for MyFirstStruct {
  fn value(&self) -> int {
    self.x
  }
}

impl MyTrait for MySecondStruct {
  fn value(&self) -> int {
    self.y
  }
}

// This function does dynamic dispatch for value() which has a runtime cost
// We call it with a Vector of MyTrait trait objects
// which have unknown size at compile time
// because they could be either MyFirstStruct or MySecondStruct
fn dynamic_dispatch(vals: Vec<MyTrait>) -> int {
  vals.iter().map(|val| val.value()).sum()
}

fn main() {
  let foo = MyFirstStruct{x: 1};
  let bar = MySecondStruct{y: 2};
  
  // To cast variable v to trait T use
  // v as T
  println!("Some calc on 1st and 2nd: {}", dynamic_dispatch(vec!(foo,bar)));
}