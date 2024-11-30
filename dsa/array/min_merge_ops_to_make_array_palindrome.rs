//Find minimum number of merge operations to make an array palindrome

fn main(){
  let mut arr = [15, 4, 15];
  let merge_ops = count_min_merge(&mut arr);
  println!("{}", merge_ops);
}

fn count_min_merge(arr: &mut [i32]) -> i32 {
  let n = arr.len();

  let mut ops = 0;

  let mut i = 0;
  let mut j = n - 1;
  while i <= j {
    //if elements are same
    if arr[i] == arr[j]{
      i += 1;
      j -= 1;
    }
    //if strating ele is greater merge last one
    else if  arr[i] > arr[j]{
      j -= 1;
      arr[j] += arr[j+1];
      ops += 1;
    }
    else{
      //merge the two starting ele
      i += 1;
      arr[i] += arr[i-1];
      ops += 1;
    }
  }
  ops
}