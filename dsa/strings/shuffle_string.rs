// You are given a string s and an integer array indices 
// of the same length. The string s will be shuffled 
// such that the character at the 
// ith position moves to indices[i] in the shuffled string.

fn main() {
  let s = "bac".to_string();
  let indices = vec![1, 0, 2];
  let res = restore_string(s, indices);
  println!("{}", res);
}

fn restore_string(s: String, indices: Vec<usize>) -> String {
  let n = indices.len();
  let mut res = vec![' '; n];  
  
  for (i, &index) in indices.iter().enumerate() {
      res[index] = s.chars().nth(i).unwrap();  
  }
  
  res.iter().collect() 
}
