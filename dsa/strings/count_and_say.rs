fn main() {
  let n = 4;
  let res = count_and_say(n);
  println!("{}", res);
}

fn count_and_say(n: i32) -> String {
  if n == 1 {
      return "1".to_string();
  }
  
  let s: String = count_and_say(n - 1);
  let mut res = String::new();
  let mut freq = 0;

  let chars: Vec<char> = s.chars().collect();  
  for i in 0..chars.len() {
    freq += 1;
    if i == chars.len() - 1 || chars[i] != chars[i + 1] {
        res.push_str(&freq.to_string());  
        res.push(chars[i]);  
        freq = 0; 
    }
  }
  res
}
