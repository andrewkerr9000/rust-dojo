// http://doc.rust-lang.org/guide-tasks.html#backgrounding-computations:-futures

#![crate_name = "futures"]

extern crate time;
use std::sync::Future;
use std::io::timer::sleep;
use std::time::duration::Duration;
use time::precise_time_s;

fn add_one_very_slowly(n: int) -> int {
  sleep(Duration::seconds(1)); // modifying this line is cheating
  n + 1
}

fn main() {
  let start = time::precise_time_s();
  let value_1 = add_one_very_slowly(7);
  let value_2 = add_one_very_slowly(13);
  println!("We added one to some numbers and got {} and {}", value_1,value_2);
  let end = time::precise_time_s();
  println!("It took {} seconds. Can you halve that time? Yes you can!", end - start);
}
