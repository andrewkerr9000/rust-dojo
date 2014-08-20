#![crate_name = "bounds"]
use std::default::Default;

struct MyFirstStruct {
  x: int
}

trait MyTrait {
  fn value(&self) -> int;
}

impl MyTrait for MyFirstStruct {
  fn value(&self) -> int {
    self.x
  }
}

impl Default for MyFirstStruct {
  fn default() -> MyFirstStruct {
    MyFirstStruct{x: 99}
  }
}

fn difference_from_default<T: MyTrait + Default>(val: T) -> int {
  let def: T = Default::default();
  def.value() - val.value()
}


fn main() {
  let foo = MyFirstStruct{x: 1};
 
  println!("Foo is {} different from a default value", difference_from_default(foo));
}