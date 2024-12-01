//Given a string s and a dictionary of n words dictionary,
// find out if "s" can be segmented into a space-separated sequence of dictionary words.

fn main() {
  let s = String::from("ilike");
  let word_dict: Vec<String> = vec!["i", "like", "sam", "sung", "samsung", "mobile"]
    .iter()
    .map(|&s| s.to_string())
    .collect();

  let res = word_break(s, word_dict);
  println!("{}", res);
}

fn word_break(s: String, word_dict: Vec<String>) -> bool {
  let  n = s.len();
  let mut dp = vec![false; n + 1];
  dp[0] = true;  

  for i in 0..=n { 
    for j in 0..i {
        if dp[j] && word_dict.contains(&s[j..i].to_string()) {
            dp[i] = true;
            break;
        }
    }
  }
  dp[n]
}
