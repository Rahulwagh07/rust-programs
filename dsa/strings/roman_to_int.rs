
fn main() {
  let s = String::from("III");
  let result = roman_to_int(s);
  println!("{}", result);
}

fn roman_to_int(s: String) -> i32 {
  let mut ans = 0;
  let mut num = 0;
  
  let s_chars: Vec<char> = s.chars().collect();  
  
  for i in (0..s_chars.len()).rev() { 
      num = match s_chars[i] {
          'I' => 1,
          'V' => 5,
          'X' => 10,
          'L' => 50,
          'C' => 100,
          'D' => 500,
          'M' => 1000,
          _ => 0,  
      };

      if 4 * num < ans {
          ans -= num;
      } else {
          ans += num;
      }
  }
  
  ans
}
 
