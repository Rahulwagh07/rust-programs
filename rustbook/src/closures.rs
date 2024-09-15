//closures are anonymous functions that you can save 
//in a variable or pass as an argument to other functions. 
//Closures can capture values from the environment 
//in which they are defined, making them flexible and powerful.



//Capturing the Environment with Closures
//3 ways
//Immutable reference.
// let list = vec![1, 2, 3];
// let only_borrows = || println!("From closure: {:?}", list);

//Mutable reference.
// let mut list = vec![1, 2, 3];
// let mut borrows_mutably = || list.push(7);

//Taking Ownership->
// use std::thread;
// let list = vec![1, 2, 3];
// thread::spawn(move || println!("From thread: {:?}", list)).join().unwrap();

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

// pub fn main() {
//     let store = Inventory {
//         shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
//     };

//     let user_pref1 = Some(ShirtColor::Red);
//     let giveaway1 = store.giveaway(user_pref1);
//     println!(
//         "The user with preference {:?} gets {:?}",
//         user_pref1, giveaway1
//     );

//     let user_pref2 = None;
//     let giveaway2 = store.giveaway(user_pref2);
//     println!(
//         "The user with preference {:?} gets {:?}",
//         user_pref2, giveaway2
//     );
// }




//Closure Type Inference and Annotation
pub fn main() {
  // Type Inference: The types are automatically inferred by the compiler
  let example_closure = |x| x;
  let s = example_closure(String::from("hello")); // `x` and return type inferred as `String`

  // type mismatch error
  // let n = example_closure(5); // Error: expected `String`, found `integer`

  // Type Annotation: Explicitly defining the types for clarity
  let expensive_closure = |num: u32| -> u32 {
      println!("calculating slowly...");
      std::thread::sleep(std::time::Duration::from_secs(2));
      num
  };
  let result = expensive_closure(10); // `num` is `u32`, return type is `u32`

  // Type Locking: Types once inferred cannot be changed
  let example_closure = |x| x;
  let s = example_closure(String::from("hello")); // Inferred as `String`

  //  type mismatch
  // let n = example_closure(5); // Error: expected `String`, found `integer`

}

//Moving Captured Values Out of Closures and the Fn Traits
// FnOnce: A closure that can be called once and may consume or move captured values out of itself.
// FnMut: A closure that can be called multiple times and may mutate captured values but does not move them out.
// Fn: A closure that can be called multiple times, does not mutate or move captured values, and may capture nothing from its environment.