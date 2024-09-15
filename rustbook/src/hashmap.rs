use core::num;
use std::{collections::{btree_map::{Keys, Values}, HashMap}, hash::Hash, io::{self, Write}};
pub fn main(){
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 20);
  scores.insert(String::from("Yellow"), 70);

  let team_name = String::from("Blue");
  let score = scores.get(&team_name).copied().unwrap_or(0);

  for (key, value) in &scores {
    println!("{key}: {value}");
  }



  //Ownership
  let field_name = String::from("Favorite color");
  let field_value = String::from("Blue");
  let mut map = HashMap::new();
  map.insert(field_name, field_value); //field_name and field_value are invalid at this point
  for (key, value) in &map {
    println!("{key}: {value}");
  }
  //println!("field name  : {}", field_name); 
  let sub = String::from("cs");
  let grade = String::from("A");
  let mut map = HashMap::new();
  map.insert(&sub, &grade);
  for (key, value) in &map {
    println!("{key}: {value}");
  }
  println!("Sub: {} Grade: {}", sub, grade); //valid


  let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    //Median and mode
    let mut numbers = vec![4, 1, 7, 3, 2, 1, 4, 4];
    let (median, mode) = median_mode(&mut numbers);
    println!("Median: {}", median);
    println!("Mode: {}", mode);

    //pig latin
    let words = ["first", "apple", "rust"];
    for word in words.iter() {
      println!("{} -> {}", word, pig_latin(word));
    }

    //Text interface for employee management
    employee_management();
}

fn median_mode(numbers: &mut Vec<i32>) -> (f64, i32) {
  numbers.sort();

  let len = numbers.len();
  let median = if len % 2 == 0 {
    (numbers[len / 2 - 1] + numbers[len / 2]) as f64 / 2.0
  } else {
    numbers[len / 2] as f64
  };

  let mut occurrences = HashMap::new();
  for &num in numbers.iter() {
    *occurrences.entry(num).or_insert(0) += 1;
  }

  let mode = occurrences.into_iter().max_by_key(|&(_, count)| count).map(|(num, _)| num).unwrap_or(0);

  (median, mode)
}

fn pig_latin(word: &str) -> String {
  let vowels = "aeiou";
  let first_char = word.chars().next().unwrap_or_default();

  if vowels.contains(first_char.to_lowercase().to_string().as_str()){
    format!("{}--hay", word)
  } else {
      let mut chars = word.chars();
      let first_consonant = chars.next().unwrap_or_default();
      format!("{}--{}ay", chars.as_str(), first_consonant)
  }
}

fn employee_management() {
  let mut company = HashMap::new();

    loop {
        let mut input = String::new();
        print!("Enter command: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        if input.starts_with("Add ") {
            let parts: Vec<&str> = input.split_whitespace().collect();
            if parts.len() >= 4 && parts[2] == "to" {
                let employee = parts[1].to_string();
                let department = parts[3..].join(" ");
                company.entry(department).or_insert_with(Vec::new).push(employee);
            } else {
                println!("Invalid command format. Use: Add <name> to <department>");
            }
        } else if input == "list all" {
            for (department, employees) in &company {
                println!("{}: {:?}", department, employees);
            }
        } else if input.starts_with("list ") {
            let department = input[5..].to_string();
            if let Some(employees) = company.get(&department) {
                println!("{}: {:?}", department, employees);
            } else {
                println!("No such department.");
            }
        } else {
            println!("Unknown command.");
        }
    }
} 