//minimum number of jumps to reach nums[n - 1]

fn main() {
  let arr = [2,3,0,1,4];
  let ans = min_jump(&arr);
  println!("{}", ans);
}

fn min_jump(arr: &[i32]) -> i32 {
  let n = arr.len();
  let mut dp = vec![-1; n];
  dp[n - 1] = 0;

  for i in (0..n - 2).rev() {
      let steps = arr[i] as usize;
      let mut res = i32::MAX;

      for j in (i + 1)..=(i + steps).min(n - 1) {
          if dp[j] != -1 {
              res = res.min(dp[j] + 1);
          }
      }

      if res != i32::MAX {
          dp[i] = res;
      }
  }

  dp[0]
}
