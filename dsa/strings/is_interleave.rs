fn main(){
  let s1 = "abcd";
  let s2 = "xyz";
  let s3 = "axbcyzd";
  let s3_2 = "abxcdyt";

  println!("{}", is_interleave(s1, s2, s3));  
   println!("{}", is_interleave(s1, s2, s3_2));   
}

fn is_interleave(s1: &str, s2: &str, s3: &str) -> bool {
  let m = s1.len();
  let n = s2.len();
  let k = s3.len();

  if m + n != k {
    return false;
  }

  let s1_bytes = s1.as_bytes();
  let s2_bytes = s2.as_bytes();
  let s3_bytes = s3.as_bytes();

  let mut i = 0; 
  let mut j = 0;
  let mut k = 0;

  while k < s3_bytes.len() {
    if i < s1_bytes.len() && s1_bytes[i] == s3_bytes[k] {
        i += 1;
    } else if j < s2_bytes.len() && s2_bytes[j] == s3_bytes[k] {
        j += 1;
    } else {
        return false;
    }
    k += 1;
  }

  i == s1_bytes.len() && j == s2_bytes.len()
}