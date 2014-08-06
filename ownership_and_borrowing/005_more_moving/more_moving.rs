#![crate_name = "moving_again"]

// A function that takes a & can take any pointer

//Modify this type signature
fn do_something(y: &int) {
  println!("Oh hai! {}", y);
}

fn main() {
  let x = box 7i;
  do_something(&*x); // Modify here
  println!("x is {}",x);
}