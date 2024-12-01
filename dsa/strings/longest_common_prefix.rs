//Write a function to find the longest common prefix string amongst an array of strings.

use std::cmp;

fn main() {
    let mut  strs: Vec<String> = vec!["flower", "flow", "flight"]
      .iter()
      .map(|&s| s.to_string())
      .collect();
    let res = longest_common_prefix(&mut strs);
    println!("{}", res);
}

fn longest_common_prefix(strs: &mut Vec<String>) -> String {
    let n = strs.len();
    strs.sort();

    let mut ans = String::from("");
    let min_str_len = cmp::min(strs[0].len(), strs[n - 1].len());

    let mut i = 0;
    while i < min_str_len && strs[0].chars().nth(i) == strs[n - 1].chars().nth(i) {
        i += 1;
    }
    ans.push_str(&strs[0][0..i]);  
    ans
}
