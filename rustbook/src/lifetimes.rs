//lifetimes -> manages refrences.. 
//specifies how long references are valid, 
//ensuring memory safety by preventing dangling reference


// pub fn main() {
//     let r;  // Outer scope

//     {
//         let x = 5;  // Inner scope
//         r = &x;     // Reference to x
//     }  // x out of scope

//     println!("r: {r}");  // Error! r reference is invalid
// }


// pub fn main() {
//   let string1 = String::from("abcd");
//   let string2 = "xyz";

//   let result = longest(string1.as_str(), string2);
//   println!("The longest string is {result}");
// }

//a is generic lifetime parameter specifies that x and y share same lifetime
//here rust ensure that the ref return under function is valid till the time funcion is called

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {  
//   if x.len() > y.len() {
//       x
//   } else {
//       y
//   }
// }

//function to always return the first parameter rather than the longest string slice, 
//we wouldn’t need to specify a lifetime on the y parameter. 

// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//   x
// }


//example of Dangling References
//if a function return a ref then its liftime should be match 
// to the parameter of the function
// if the function refrencing a value that is in the function that value
//will go out of the scope at the funcion end 

// fn longest<'a>(x: &str, y: &str) -> &'a str {
//   let result = String::from("really long string");
//   result.as_str()  //referencing local variable
// }


//lifetimes and structs

struct ImportantExcerpt<'a> {
  part: &'a str,
}

fn main() {
  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = novel.split('.').next().unwrap();
  let i = ImportantExcerpt {
      part: first_sentence,
  };
}

//Lifetime Elision Rules
//there are some situation where there is no need to write lifetime annotations
//bcz rust complier automaticaly inference that 

fn first_word(s: &str) -> &str {  //complier infer that input and output have same lifetime
  let bytes = s.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }
  &s[..]
}


//Lifetime Annotations in Methods
//if lifetimes involved in struct methods then decalre it after impl block
impl<'a> ImportantExcerpt<'a> { //here complier infer that lifetime of return value mathches the lifetime of "self"
  fn announce_and_return_part(&self, announcement: &str) -> &str {
      println!("Attention please: {}", announcement);
      self.part
  }
}


//static lifetime -> affected lifetime live for entire duration of the program
use std::fmt::Display;

/// The generic parameter `T` must implement the `Display` trait so it can be printed.
/// The lifetime `'a` ensures the returned string slice is valid as long as both `x` and `y`.
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
