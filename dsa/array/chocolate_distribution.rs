// Given an array arr[] of n integers where arr[i] represents the number 
//of chocolates in ith packet. Each packet can have a variable number of chocolates. 
//There are m students, the task is to distribute chocolate packets such that: 

// Each student gets exactly one packet.
// The difference between the maximum and minimum number of chocolates 
//in the packets given to the students is minimized.

fn main() {
  let arr = [7, 3, 2, 4, 9, 12, 56];
  let m = 5;
  let n = arr.len();

  if n < m {
      println!("Not possible");
      return;
  }

  let mut arr = arr.to_vec(); 
  arr.sort();

  let mut min_diff = i32::MAX; 
  for i in 0..=n - m {
      let diff = arr[i + m - 1] - arr[i];
      if diff < min_diff {
          min_diff = diff;
      }
  }

  println!("Minimum diff: {}", min_diff); 
}

