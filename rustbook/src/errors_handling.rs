use std::{fs::File, io::ErrorKind};
use std::io::{self, Read};
pub fn main() {
  let username  =  read_username_from_file().unwrap();
  println!("Username read from file: {}", username);
}

fn match_to_handle_result() {
  let file_path = "/hello.txt";
  let greeting_file_result = File::open(file_path);

  //usign match to handle result
  match greeting_file_result {
      Ok(file) => {
          println!("Successfully opened the file:");
      }
      Err(error) => {
          eprintln!("Problem opening the file '{}': {:?}", file_path, error);
      }
  }
}
fn error_handling_with_unwrap_or_else() {
  let file = File::open("hello.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create("hello.txt").unwrap_or_else(|error| {
        panic!("Problem creating the file: {error:?}");
      })
    } else {
      panic!("Problem opening the file: {error:?}");
    }
  });
}

fn error_handling_with_unwrap() {
  let greeting_file = File::open("hello.txt").unwrap();
}

fn error_handling_with_expect() {
  let greeting_file = File::open("hello.txt")
    .expect("hello.txt should be included in this project");
}
 
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
