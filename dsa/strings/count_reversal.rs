//Given a string S consisting of only opening and 
//closing curly brackets '{' and '}', find out the
// minimum number of reversals required to convert the string into a balanced expression.
//A reversal means changing '{' to '}' or vice-versa.

fn main() {
  let s = String::from("}{{}}{{{");
  let count = count_reversal(s);
  println!("{}", count); 
}

fn count_reversal(s: String) -> i32 {
  let mut stack: Vec<char> = Vec::new();
  let n = s.len();

  if n % 2 == 1 {
      return -1;
  }

  for i in 0..n {
      let ch = s.chars().nth(i).unwrap();
      if ch == '{' {
          stack.push(ch);
      } else {
          if !stack.is_empty() && stack.last() == Some(&'{') {
              stack.pop();
          } else {
              stack.push(ch);
          }
      }
  }
  let mut a = 0; 
  let mut b = 0; 
 
  while !stack.is_empty() {
      let ch = stack.pop().unwrap();
      if ch == '{' {
          a += 1;
      } else {
          b += 1;
      }
  }
  let count = (a + 1) / 2 + (b + 1) / 2;
  count
}
