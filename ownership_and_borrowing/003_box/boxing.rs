#![crate_name = "boxing"]

// Boxing places data on heap instead of stack
// A boxed pointer is also known as an owned pointer
// The keyword box boxes
// A boxed pointer to a type T has type Box<T>
// Boxed pointers can also be dereferenced with *
// ~ is deprecated
// A data structure that contains a type of unknown size needs to box it, as the pointer is of known size
// An example is a recursive structure

enum MyLinkedList<T> {
  Nil,
  Cons(T,<MyLinkedList<T>)
}

fn main() {
  let list = Cons(2i, Cons(6i, Nil));
  println!("We made a list!");
  print_list(list);
  
  // Functions can be nested
  fn print_list(list: MyLinkedList<int>) {
    match list {
      Nil => println!("End of list"),
      Cons(value, next) => {
	println!("Contains value {}", value);
	print_list(next);
      }
    }
  }
}