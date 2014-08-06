#![crate_name = "mutable_references"]

// &mut is mutable but not aliasable

fn main() {
  let mut data = 4i;
  println!("data starts as {}", data);
  data += 1;
  println!("data has been mutated to {}", data);
  {
    let ref_data = &mut data;
    println!("mutable ref to data points to {}", ref_data);
    *ref_data +=2;
    println!("data accessed via ref_data is now {}", ref_data);
    println!("data is {}", data);
  }
  println!("data ends as {}", data);
}