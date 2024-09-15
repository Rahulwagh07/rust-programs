
//without generics
fn largest_i32(list: &[i32]) -> &i32 { /* logic */ }
fn largest_char(list: &[char]) -> &char { /* logic */ }

//With Generics
fn largest<T: PartialOrd>(list: &[T]) -> &T { /* logic */ }


fn largest<T: PartialOrd>(list: &[T]) -> &T {
  let mut largest = &list[0];
  for item in list {
    if item > largest {
      largest = item;
    }
  }
  largest
}


struct Point<T> {
  x:T,
  y:T,
}

struct Point<T, U> {
  x:T,
  y:U,
}

enum Option<T> {
  Some(T),
  None,
}

enum Result<T,E> {
  Ok(T),
  Err(E),
}

//Method definitions
impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}