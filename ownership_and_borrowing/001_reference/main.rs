#![crate_name = "references"]

// Rust is call-by-value
// If a data structure is sufficiently huge we may want to pass is around by references
// We could check performance implications with the built in micro-benchmarking functionality
// A reference, aka borrowed pointer is immutable and aliasable
// This is the most common pointer
// the sigil & denotes a reference
// the sigil * dereferences a reference, but dereferencing can be implicit


struct ReallyBigDataStructure {
  x:int
}

fn access_really_big_data_structure(big: &ReallyBigDataStructure) {
  println!("The really big data structure wraps the value {}", (*big).x);
  println!("Do we need to explicitly deref to acces {}?", big.x);
}

fn access_really_big_part_of_really_big_data_structure(big: &int) {
  println!("The really big part of the really big data structure wraps the value {}", *big);
println!("Do we need to explicitly deref to acces {}?", big);
}

// Plz to modfy only here kthnx 
fn main() {
  let like_totally_huge = ReallyBigDataStructure {x: 3};
  access_really_big_data_structure(&like_totally_huge);
  access_really_big_part_of_really_big_data_structure(&like_totally_huge.x);
}