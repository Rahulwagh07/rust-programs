//a typical test fuction has 3 part
// 1)setup
// 2) run code
// 3) assertions

pub fn add(left: usize, right: usize) -> usize {
  left + right
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
      let result = add(2, 2);
      assert_eq!(result, 4);  
  }
}

// Result in tests
#[test]
fn it_works() -> Result<(), String> {
    if 2 + 2 == 5 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}
