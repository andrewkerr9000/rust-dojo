#![crate_name = "destructors"] 

// Manual test task: by introducing code blocks but otherwise changing it

// Drop is called when variable goes out of scope to clean up. It cannot be called automatically.

// make the program print
// 1
// 2
// 3

// ignore the warning

struct Test(int);

impl Drop for Test {
  // drop frees memory, closes files generally cleans up after itself
  fn drop(&mut self) {
    let Test(x) = *self;
    println!("{}",x);
  }
}

//Wrap code in code blocks, but do not move lines up or down
#[allow(unused_variable)]
fn main() {
  let foo = box Test(2);
  let bar = Test(1);
  println!("3");
}