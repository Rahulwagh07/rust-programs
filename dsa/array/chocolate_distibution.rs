//Given an array arr[] of positive integers, 
// where each value represents the number of chocolates in a packet. 
// Each packet can have a variable number of chocolates. 
// There are m students, 
// the task is to distribute chocolate packets among m students such that -
// i. Each student gets exactly one packet.
// ii. The difference between maximum number of chocolates

use std::cmp;

fn main(){
  let mut chocolate = [7, 3, 2, 4, 9, 12, 56];
  let students: usize = 3;
  let min_diff = find_min_diff(&mut chocolate, students);
  println!("{}", min_diff);
}

fn find_min_diff(chocolate: &mut [i32], students: usize) -> i32 {
  let n = chocolate.len();
  if students == 0 || n == 0 {
    return 0;
  }
  if students > n{
    return -1;
  }
  chocolate.sort();
  let mut  min_diff = i32::MAX;

  for i in 0..n-students+1{
    let diff = chocolate[i+students-1] - chocolate[i];
    min_diff = cmp::min(diff, min_diff);
  }
  min_diff

}