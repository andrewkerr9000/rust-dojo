#![crate_name = "moving_ownership"]

// There can be only one owner

fn main() {
  let foo = box 4i;
  println!("foo is {}", foo);
  {
    let bar = &foo; // Plz to modfy here kthnx. There are several things you could do, with different types of bar
    println!("bar is {}", bar);
  }
  println!("foo is {}", foo);
}