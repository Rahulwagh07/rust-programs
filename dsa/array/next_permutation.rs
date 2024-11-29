fn main() {
  let mut arr = vec![1, 1, 5];
  println!("{:?}", arr);
  next_permutation(&mut arr);
  println!("{:?}", arr);
}

fn next_permutation(arr: &mut [i32]) {
  let n = arr.len();
  if n <= 1 {
      return;
  }

  // Find the last increasing index
  let mut last_inc = -1;
  for i in (0..n - 1).rev() {
      if arr[i] < arr[i + 1] {
          last_inc = i as i32;
          break;
      }
  }

  if last_inc == -1 {
      reverse(arr, 0, (n - 1) as i32);
      return;
  }

  //Find the smallest number larger than arr[lastinc]
  let mut index = (last_inc + 1) as usize;
  for i in ((last_inc + 1) as usize)..n {
      if arr[i] > arr[last_inc as usize] && arr[i] <= arr[index] {
          index = i;
      }
  }

  //swap the two elment
  swap(arr, last_inc as usize, index);

  reverse(arr, (last_inc + 1) as i32, (n - 1) as i32);
}

fn swap(arr: &mut [i32], i: usize, j: usize) {
  let temp = arr[i];
  arr[i] = arr[j];
  arr[j] = temp;
}

fn reverse(arr: &mut [i32], start: i32, end: i32) {
  let mut start = start as usize;
  let mut end = end as usize;
  while start < end {
      swap(arr, start, end);
      start += 1;
      end -= 1;
  }
}
