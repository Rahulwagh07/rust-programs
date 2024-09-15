
pub fn main(){
  let s1 = "Initial contents".to_string();
  let s2 = String::from("Initial contents");
 
  println!("s1: {}", s1);
  println!("s2: {}", s2);

  let mut s = String::from("firstname");

  s.push_str("lastname");
  println!("{}", s);

  //adding single char
  s.push('!');
  println!("After push: {}", s);

  //Concatenating with + 
 // Ownership and Borrowing: The + operator in Rust takes 
 //ownership of the s1 string and appends the
 //&str reference to s2 to it. This means s1 is no 
 //longer accessible after the concatenation.

//String Slices: &s2 is a string slice (&str), 
//which is a view into the string data. The + 
//operator can only append &str to String.
  let s1 = String::from("Hello, ");
  let s2 = String::from("Rust!");
  let s3 = s1 + &s2;
 // println!("s1: {}", s1); //s1 is no longer valid here
  println!("s3: {}", s3);

  //Concatenating wiht the format! macro
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");
  let s = format!("{}--{}--{}", s1, s2, s3);
  println!("Formatted string: {}", s);

  //Indexing and Slicing stings
  let s = String::from("abcdef");

  let slice = &s[0..4];
  println!("Slice (bytes 0..4: {}", slice);

  //Iterate over characters
  for c in s.chars(){
    println!("{}", c);
  }

  //Iterate over raw bytes
  for b in s.bytes(){
    println!("{}", b);
  }

}