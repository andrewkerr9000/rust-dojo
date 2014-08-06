#![crate_name = "lifetimes"] 

// If a function returns a reference the compiler needs more information to calculate the lifetime of the returned values
// This is (partly) to prevent a dangling pointer
// Inference rules have been proposed to (mostly) eleminate the need to do this

fn last_element<'r>(data: &'r Vec<int>) -> &'r int {
   data.last().unwrap()
}

fn main() {
  let &last = last_element(&vec!(1i,2,3,4));
  println!("last element was {}", last);
}