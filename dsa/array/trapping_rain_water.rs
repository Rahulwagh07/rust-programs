//Given n non-negative integers representing an 
//elevation map where the width of each bar is 1, 
//compute how much water it can trap after raining.

//water level -> min(leftmax, rightmax)
//trapped water -> waterlevel - height

use std::cmp;

fn main(){
  let height = [4,2,0,3,2,5];
  let res = get_trapped_water(&height);
  println!("{}", res);
}

fn get_trapped_water(height: &[i32]) -> i32 {
  let n = height.len();
  if n == 0{
    return 0;
  }
  let mut rightmax = vec![0; n];
  let mut leftmax = vec![0; n];

  rightmax[n-1] = height[n-1];
  for i in (0..n-1).rev(){
    rightmax[i] = cmp::max(height[i], rightmax[i+1]);
  }

  leftmax[0] = height[0];
  for i in 1..n{
    leftmax[i] = cmp::max(height[i], leftmax[i-1]);
  }

  let mut trapped_water = 0;
  for i in 0..n{
    let waterlevel = cmp::min(leftmax[i], rightmax[i]);
    trapped_water += waterlevel - height[i];
  }
  println!("{:?}", rightmax);
  trapped_water
}