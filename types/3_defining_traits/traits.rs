#![crate_name = "traits"]

struct MyFirstStruct {
  x: int
}

// define MyTrait

impl MyTrait for MyFirstStruct {
  fn value(&self) -> int {
    self.x
  }
}

fn main() {
  // This is boxed as the type is a trait object, which has unknown size
  let foo: Box<MyTrait> = box MyFirstStruct{x:3};
  println!("Value is {}", foo.value());
}