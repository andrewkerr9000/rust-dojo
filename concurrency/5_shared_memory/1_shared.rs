// Tasks are owned closures: they own all variables they capture
// To share a variable without cloning it we need to wrap it in an atomic reference counted pointer or Arc
// and then clone the Arc
// Arc implements the deref traits, and can therefore be derefed either using * or implicitly
// http://doc.rust-lang.org/std/sync/struct.Arc.html

#![crate_name = "shared_memory"]
use std::sync::Arc;

fn main() {
  let foo = vec!(1i,2,3);


  spawn(proc() {
    println!("I captured {}", foo);
  });


  spawn(proc() {
    println!("I also captured {}", foo);
  });
}