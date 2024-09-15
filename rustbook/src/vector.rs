 
#[derive(Debug)]
enum SpreadsheetCell {
  Int(i32),
  Float(f64),
  Text(String),
}

pub fn main(){
  let mut v = vec![1, 2, 3];
  let first = v[0];
  println!("Frst ele is {}", first);
  v.push(4);
  for i in &v {
    println!("{i}")
  }
  let sec_ele = v.get(1);
  match  sec_ele {
      Some(&value) => println!("The ele is {value}"),
      None => print!("There is no ele"),
  }

  //mutable iteration
  let mut salary = vec![1000, 2000, 3000];
  for i in &mut salary {
    *i += 5000;
    println!("{i}");
  }

  //Multiple types
  let row = vec![
      SpreadsheetCell::Int(3),
      SpreadsheetCell::Text(String::from("blue")),
      SpreadsheetCell::Float(10.12),
  ];
  let second = row.get(1);
  println!("{:?}", second);

  //Droppint vector: When a vector goes out of scope,
  // its memory and elements are automatically freed:
}