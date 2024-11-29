use std::cmp;

fn main(){
  //find the sum of contigous subarray with maximum sum
  let mut arr =  [-2, -4];
  let res = max_subarray_sum(&mut arr);
  println!("{}", res);
}

fn max_subarray_sum(arr: &mut[i32]) -> i32{
  let n = arr.len();
  let mut max_sum = i32::MIN;
  let mut curr_sum = 0;
  
  for i in 0..n {
    curr_sum = cmp::max(arr[i], curr_sum+ arr[i]);
    max_sum = cmp::max(max_sum, curr_sum);
  }
  max_sum
}