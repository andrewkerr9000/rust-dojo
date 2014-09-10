// To share mutable memory we must wrap the variable in a mutex (or rwlock)
// which we then wrap in an Arc

// This prevents data races
// It does not prevent deadlock or other race condition

// http://doc.rust-lang.org/std/sync/struct.Mutex.html

// using try instead of spawn just to ensure 

#![crate_name = "shared_mutable_memory"]

use std::sync::{Arc,Future,Mutex};

fn main() {
  let foo = Arc::new(Mutex::new(vec!(1i,2,3)));

  let bar = foo.clone();
  let mut f1 = Future::spawn(proc() {
    let mut val = bar.lock();
    println!("I captured {}", *val);
    val.push(4);
  });

  let baz = foo.clone();
  let mut f2 = Future::spawn(proc() {
    let mut val = baz.lock();
    println!("Whereas I captured {}", *val);
    val.push(5);
  });

  f1.get();
  f2.get();

  println!("Final vector is {}", *(foo.lock()));
}