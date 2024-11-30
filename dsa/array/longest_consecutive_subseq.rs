//Given an array arr of non-negative integers. 
//Find the length of the longest sub-sequence such that elements in the subsequence are 
//consecutive integers, the consecutive numbers can be in any order.

use std::cmp;
fn main(){
  let mut arr = [2, 6, 1, 9, 4, 5, 3];
  let ans = solve(&mut arr);
  println!("{:?}", ans);
}

fn solve(arr: &mut[i32]) -> i32 {
  let mut curr_count = 1;
  let mut max_count = 1;
  let n = arr.len();
  arr.sort();
  for i in 1..n{
    if arr[i] == arr[i-1] + 1{
      curr_count +=1;
    } else if arr[i] != arr[i-1]{
      curr_count = 1;
    }
    max_count = cmp::max(curr_count, max_count);
  }

  max_count
}