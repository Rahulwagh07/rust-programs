use std::io;
pub fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}

pub fn data_types() {
    // Floating points
    let x = 2.0;  
    let y: f32 = 3.0;  
    println!("x value is {x}");
    println!("y value is {y}");
}

 
pub fn numeric_operations() {
    let _sum = 5 + 10;
    println!("sum: {_sum}");
    let _difference = 95.5 - 4.3;
    println!("{_difference}");
    
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3;  
    let _remainder = 43 % 5;

    let t = true;
    println!("{t}");
}

 
pub fn char_types() {
    let c = 'z';
    let z: char = 'R';  
    let heart_eyed_cat = '😻';
    println!("{c}");
    println!("{z}");
    println!("{heart_eyed_cat}");
}

 
pub fn tuples_and_array() {
    let my_tuple = (74, "LFG", 2.5, 'A');
    println!("{}", my_tuple.1);

    let (roll_no, wryd, index, first_char) = my_tuple;
    println!("{roll_no} {wryd} {index} {first_char}");

    let numbers = [7, 8, 90, 90];
    let num = numbers[1];
    println!("{num}");
}
pub fn invalid_array_ele_access(){
  let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
