// To share mutable memory we must wrap the variable in a mutex (or rwlock)
// which we then wrap in an Arc

// This prevents data races
// It does not prevent deadlock or other race condition

// http://doc.rust-lang.org/std/sync/struct.Mutex.html

// using try instead of spawn just to ensure 

#![crate_name = "shared_mutable_memory"]

use std::sync::{Arc,Future,Mutex};

fn main() {
  let mut foo = Arc::new(vec!(1i,2,3));

  let mut bar = foo.clone();
  let mut f1 = Future::spawn(proc() {
    println!("I captured {}", *bar);
    bar.push(4);
  });

  let mut baz = foo.clone();
  let mut f2 = Future::spawn(proc() {
    println!("Whereas I captured {}", *baz);
    baz.push(5);
  });

  f1.get();
  f2.get();

  println!("Final vector is {}", *foo);
}