// http://doc.rust-lang.org/guide-tasks.html#communication
// http://doc.rust-lang.org/std/comm/index.html

#![crate_name = "message_passing"]
use std::rand::distributions::{IndependentSample, Range};

fn main() {
  spawn(proc() {
    loop {
      let val = 0i; // get this from the other task
      match val {
	0 => {
	  println!("Recieved: {}, exiting",val);
	  break;
	},
	n => println!("Recieved: {}",n)
      }
    }
  });

  spawn(proc() {
    let between = Range::new(0i, 20);
    let mut rng = std::rand::task_rng();
    loop {
      let randon_number = between.ind_sample(&mut rng);
      // send this to the other task
    }
  });
}