fn main() {
  let nums = vec![1, 2, 2, 3, 4, 5];
  let res = find_duplicate(&nums);
  println!("{}", res);
}

fn find_duplicate(nums: &[i32]) -> i32 {
  let mut slow = nums[0] as usize;
  let mut fast = nums[0] as usize;

  loop {
      slow = nums[slow] as usize;
      fast = nums[nums[fast] as usize] as usize;
      if slow == fast {
          break;
      }
  }

  slow = nums[0] as usize;
  while slow != fast {
      slow = nums[slow] as usize;
      fast = nums[fast] as usize;
  }
  
  slow as i32
}
