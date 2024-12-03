//find and return the index of the first row that contains the maximum number of 1s. 

fn main() {
  let arr: &[&[i32]] = &[
        &[0, 1, 1, 1],
        &[0, 0, 1, 1],
        &[1, 1, 1, 1],
        &[0, 0, 0, 0],
    ];
  let row = row_with_max_1s(&arr);
  println!("{}", row);
}

fn row_with_max_1s(arr: &[&[i32]]) -> i32 {
  let n = arr.len();
  let mut row_index = -1;
  let mut max_count = 0;

  for row in 0..n {
      let mut count = 0;

      for &col in arr[row] {
          if col == 1 {
              count += 1;
          }
      }

      if count > max_count {
          max_count = count;
          row_index = row as i32;
      }
  }

  row_index
}