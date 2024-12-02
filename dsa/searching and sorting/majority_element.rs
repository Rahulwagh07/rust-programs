use std::collections::HashMap;

fn main(){
  let nums: Vec<i32> = vec![2,2,1,1,1,2,2];
  let res = majority_element(nums);
  println!("{}", res);
}

fn majority_element(nums: Vec<i32>) -> i32{
  let mut num = 0;
  let n = nums.len();
  let mut map: HashMap<i32, i32> = HashMap::new();
  for i in 0..n{
    *map.entry(nums[i]).or_insert(0) += 1;
  }
 
  for (&key, &value) in map.iter() {
    if value > n as i32 / 2 {
        num = key;
        break;
    }
  }
  num
}
