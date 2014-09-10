// http://doc.rust-lang.org/guide-tasks.html#basics
// http://doc.rust-lang.org/std/task/fn.spawn.html

#![crate_name = "tasks"]

fn print_one() {
  for n in range (0u,10) {
    println!("{}",n);
  }
}



fn main() {
  for i in range (0u,10) {
    print_one();
  }
  println!("Make this interweave the counts, rather than complete one, then do the next, then the next ...");
}