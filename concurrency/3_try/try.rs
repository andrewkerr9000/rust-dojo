// the fail! macro kills the current task
// assert! calls fail if its condition is false

// try spawns a task in the current thread
// http://doc.rust-lang.org/guide-tasks.html#handling-task-failure
// http://doc.rust-lang.org/std/task/fn.try.html

// it returns a Result, which is an option type that can be Ok(result) or Err(error)
// Result can be used in a match block. It also has higher order functions.
// Where Java may use a checked exception Rust returns a Result
// http://doc.rust-lang.org/std/result/type.Result.html
// http://rustbyexample.com/result.html

use std::rand;
use std::task::try;

fn really_reliable_computation() -> uint {
  for n in range (0u, 10) {
    if rand::random::<bool>() {
      println!("The network is reliable!");
      fail!();
    }
  }
  12
}

// Modify me
fn hammer_it_until_it_works() -> uint {
  try(really_reliable_computation).unwrap_or_else(|_| hammer_it_until_it_works())
}

fn main() {
  let foo = hammer_it_until_it_works();
  println!("You are a winrar! The reliable computation successfully returned {}",foo);
}