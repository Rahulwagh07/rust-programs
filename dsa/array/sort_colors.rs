fn main() {
  let mut numbers = vec![2, 2, 2, 1, 1, 0];  
  sort_colors(&mut numbers); 
  println!("{:?}", numbers);  
}

fn sort_colors(numbers: &mut Vec<i32>) {
  let mut arr = vec![1; numbers.len()];  
  let mut start_index = 0;
  let mut end_index = numbers.len() - 1;

  for i in 0..numbers.len() {
      if numbers[i] == 0 {
          arr[start_index] = 0;
          start_index += 1;
      } else if numbers[i] == 2 {
          arr[end_index] = 2;
          end_index -= 1;
      }
  }

  
  for j in 0..arr.len() {
      numbers[j] = arr[j];
  }
}
