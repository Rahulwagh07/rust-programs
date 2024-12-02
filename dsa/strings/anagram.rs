//Given two strings s and p, return an array of all the start indices of p's 
// anagrams
// in s. You may return the answer in any order.

fn main() {
  let s = String::from("cbaebabacd");
  let p = String::from("abc");
  let result = find_anagrams(s, p);
  println!("{:?}", result);  
}


fn find_anagrams(s: String, p: String) -> Vec<i32> {
  let mut list = Vec::new();
  let n = s.len();
  let m = p.len();

  if n < m {
      return list;
  }

  //convert p to sorted vec of chars
  let mut p_chars: Vec<char> = p.chars().collect();
  p_chars.sort();

  for i in 0..=n - m {
      let sub: Vec<char> = s[i..i + m].chars().collect();
      let mut sub_sorted = sub.clone();
      sub_sorted.sort();
      if sub_sorted == p_chars {
          list.push(i as i32);
      }
  }

  list
}

 