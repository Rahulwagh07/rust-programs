pub fn main() {
  let s = String::from("hello");  
  takes_ownership(s);                                  

  let x = 5;                      
  makes_copy(x);                           

} 

fn takes_ownership(some_string: String) {
  println!("{some_string}");
}

fn makes_copy(some_integer: i32) { 
  println!("{some_integer}");
} 
pub fn ref_and_borrowing() {
  let s1 = String::from("hello");

  let len = calculate_length(&s1);

  println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
  s.len()
}