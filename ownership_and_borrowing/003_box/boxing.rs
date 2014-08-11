#![crate_name = "boxing"]

// Boxing places a variable on the heap in its own location
// If a variable x contains a boxed variable it contains a pointer to the data in y instead of the data to y
// In Java objects are always boxed which can have a detrimental performance impact as sequential RAM access is faster than random access

// A boxed pointer is also known as an owned pointer
// The keyword "box" boxes
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