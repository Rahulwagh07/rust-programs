fn main() {
  let s = "12";
  let even_digits = ['0', '2', '4', '6', '8'];
  let mut total_count = 0;

  for i in 0..s.len() {
      if even_digits.contains(&s.chars().nth(i).unwrap()) {
          total_count += i + 1;
      }
  }

  println!("{}", total_count);
}