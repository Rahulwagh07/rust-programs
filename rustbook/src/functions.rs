use std::io;
pub fn sum(a:u16, b:u16) -> u16{
  a+b
}

pub fn check_number(){
  println!("Enter an number");
  let mut input  = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read input");
  let number: i32  = match input.trim().parse() {
      Ok(num)=> num,
      Err(_)=>{
        println!("Invalid input");
        return;
      }
  };

  if number > 0 {
    println!("The number {} is positive:", number);
  } else if number < 0 {
    println!("The number {} is negative:", number);
  } else {
    println!("The number is Zero");
  }

}

pub fn countdown(){
   println!("Enter a positive num");
   let mut input  = String::new();
   io::stdin().read_line(&mut input).expect("Failed to read input");
   let countdown_start: i32 = match input.trim().parse() {
       Ok(num) if num > 0 => num,
       _ => {
           println!("Invalid input. Please enter a positive integer.");
           return;
       }
   };

   println!("Countdown:");
   let mut countdown = countdown_start;
   while countdown >= 0 {
       println!("{}", countdown);
       countdown -= 1;
   }
}

pub fn sum_all_num(){
  println!("Enter a positive integer for summation:");
  let mut input  = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read line");
  let sum_up_to: u32 = match input.trim().parse() {
      Ok(num) if num > 0 => num,
      _ => {
          println!("Invalid input. Please enter a positive integer.");
          return;
      }
  };

  let mut sum = 0;
  let mut current = 1;
  loop {
      if current > sum_up_to {
          break;
      }
      sum += current;
      current += 1;
  }
  println!("Sum of numbers up to {} is: {}", sum_up_to, sum);
}