//Given an input string (s) and a pattern (p), 
// implement wildcard pattern matching with support for '?' and '*' where:

// '?' Matches any single character.
// '*' Matches any sequence of characters (including the empty sequence

fn main() {
  let s = String::from("aa");
  let p = String::from("*");
  let res = is_match(&s, &p);
  println!("{}", res);
}

fn is_match(s: &str, p: &str) -> bool {
  let n = s.len();
  let m = p.len();

  let mut dp = vec![vec![false; m + 1]; n + 1]; // vector with n+1 rows, each containing m+1 col

  // init
  dp[0][0] = true;
  for i in 1..=n {
      dp[i][0] = false;
  }
  for j in 1..=m {
      if p.chars().nth(j - 1).unwrap_or('\0') == '*' {
          dp[0][j] = dp[0][j - 1];
      }
  }

  for i in 1..=n {
      for j in 1..=m {
          let sc = s.chars().nth(i - 1).unwrap_or('\0');
          let pc = p.chars().nth(j - 1).unwrap_or('\0');
          if sc == pc || pc == '?' {
              dp[i][j] = dp[i - 1][j - 1];
          } else if pc == '*' {
              dp[i][j] = dp[i][j - 1] || dp[i - 1][j];
          } else {
              dp[i][j] = false;
          }
      }
  }

  dp[n][m]
}
