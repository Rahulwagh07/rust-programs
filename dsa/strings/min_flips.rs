use std::cmp;

fn main(){
  let s = String::from("0001010111");
  let flips = min_flips(s);
  println!("{}", flips);
}

fn min_flips(s: String) -> i32 {
  let mut count_1 = 0; // for pattern starting with 0
  let mut count_0 = 0; // for pattern starting with 1

  for (i, ch) in s.chars().enumerate() {
      if i % 2 == 0 {
          if ch == '1' {
              count_1 += 1; // pattern  1 expects 0, flip 1->0
          } else {
              count_0 += 1; // pattenr 2 expects 1 , flip 0 -> 1
          }
      } else {
          if ch == '0' {
              count_1 += 1; // pattenr 1 expects 1 , flip 0 -> 1
          } else {
              count_0 += 1; // pattern  2 expects 0, flip 1->0
          }
      }
  }
  cmp::min(count_0, count_1)
}