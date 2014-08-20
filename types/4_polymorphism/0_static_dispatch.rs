#![crate_name = "static"]

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

// This function uses static dispatch for value(), so has no more overhead than any other function call
// Therefore this is prefered if possible
// Compiler creates one version of the function for each type T implementing MyTrait
fn static_dispatch(val: T) -> int {
  val.value() * 2
}

fn main() {
  let foo = MyFirstStruct{x: 1};
  let bar = MySecondStruct{y: 2};
  
  println!("Some other calc on 1st: {}", static_dispatch(foo));
  println!("Some other calc on 2st: {}", static_dispatch(bar));
}