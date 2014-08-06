#![crate_name = "destructors"] 

// Manual test task: by introducing code blocks but otherwise changing it

// make the program print
// 1
// 2
// 3

// ignore the warning

struct Test;

impl Drop for Test {
  // drop frees memory, closes files generally cleans up after itself
  fn drop(&mut self) {
    println!("2");
  }
}

//Add the following characters:
// {
// }
#[allow(unused_variable)]
fn main() {
  let foo = Test;
  println!("1");
  println!("3");
}