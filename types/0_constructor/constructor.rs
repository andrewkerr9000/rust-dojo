#![crate_name = "constructors"]

// Structs are laid out in memory like C structs, with non-boxed member values stored in contiguous blocks

// Other types of struct include tuple struct without named members and unit structs with no members.

// Methods are added to structs in separate impl blocks
// Static / constructor methods do not have a reference to the instance

struct MyStruct {
  x: int
}

impl MyStruct {
  fn new(y: int) -> MyStruct {
    MyStruct{x: y}
  }
}

fn main() {
  let foo = MyStruct::new(1);
  println!("{}",foo.x);
}