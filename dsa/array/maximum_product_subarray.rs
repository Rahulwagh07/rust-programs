//Given an integer array nums, find a 
// subarray
// that has the largest product, and return the product.

//brute force- all subarray
// using kadanes algo

use std::cmp;

fn main(){
  let arr = [2,3,-2,4];
  let res = max_product(&arr);
  println!("max product is: {:?}", res);
}

fn max_product(arr: &[i32]) -> i32 {
  let mut pre = 1;
  let mut suff = 1;

  let n = arr.len();
  let mut product = i32::MIN;

  for i in 0..n{
    if pre == 0{
      pre = 1;
    }
    if suff == 0{
      suff = 1;
    }
    pre = pre * arr[i];
    suff = suff * arr[n-i-1];
    product = cmp::max(product, cmp::max(pre, suff));
  }
  product
}