// Find out the minimum possible difference between the 
//height of the shortest and tallest towers after
// you have modified each tower.

use std::cmp;

fn main(){
  let mut arr = [1, 5, 8, 10];
  arr.sort();
  println!("{:?}", arr);
  let k = 2;

  let n = arr.len();
  let mut res = arr[n-1] - arr[0];
  let smallest = arr[0] + k;
  let largest = arr[n-1] - k;

  let mut min;
  let mut max;

  for i in 0..n-1{
    min = cmp::min(smallest, arr[i+1] - k);
    max = cmp::max(largest, arr[i]+k);
    if min < 0{
      continue;
    }
    res = cmp::min(res, max-min);
  }
  
  println!("{:?}", res);
}