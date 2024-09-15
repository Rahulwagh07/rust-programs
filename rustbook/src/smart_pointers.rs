// smart pointers are a type of data structure that not only act as a pointer but also manage
// additional data and functionality. They provide more capabilities 
//than regular pointers, such as automatic memory management,
// reference counting, and ownership tracking

//Box<T>
//Provides ownership of heap-allocated memory.
//Used for storing data on the heap rather than the stack. 
//What remains on the stack is the pointer to the heap data.
//It’s useful when you have a large amount of data or need a fixed-size type at runtime.

// Use Cases:
// Unknown Size: When dealing with types whose size cannot be known at compile time.
// Large Data: For large data to avoid copying, as only the pointer is copied.
// Trait Objects: To work with values implementing a trait without knowing the specific type.


// Recursive Types and Box<T>:
//A Cons list is a linked list structure where each element points to the next, with Nil indicating the end.
use std::fmt;
pub enum List {
  Cons(i32, Box<List>), //Represents a node in the list containing an integer (i32)   
  Nil,                  //and a pointer (Box<List>) to the next node in the list.
}

impl fmt::Debug for List {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
          List::Cons(value, next) => write!(f, "{:?} -> {:?}", value, next),
          List::Nil => write!(f, "Nil"),
      }
  }
}
pub fn create_list() -> List {
  List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil)))))) 
}

pub fn main() {
  let list = create_list();
  // To print `list`, implement the `Debug` trait for `List`
  println!("{:?}", list);
}