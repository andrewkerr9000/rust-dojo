#![crate_name = "extensions"]

// Shockingly no one thought to include a method on int to increment it by seventeen. Let's fix that!

// We need to define and implement a trait.

trait IncrementableBySeventeen {
  fn increment_by_seventeen(&mut self);
}

impl IncrementableBySeventeen for int {
  fn increment_by_seventeen(&mut self) {
    *self += 17;
  }
}

fn main() {
  let mut foo: int = 3;
  foo.increment_by_seventeen();
  println!("foo is now {}",foo);
}