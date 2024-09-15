use std::io::{self, Write};
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// Function to add one to an Option<i32> value
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    println!("Value in cents: {}", value_in_cents(coin));

    let number = Some(5);
    let incremented_number = plus_one(number);
    println!("Number plus one: {:?}", incremented_number);

    // match with catch-all pattern
    print!("Enter the dice roll value: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let dice_roll: u8 = input.trim().parse().expect("Please enter a valid number");
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {
        println!("Adding a fancy hat!");
    }

    fn remove_fancy_hat() {
        println!("Removing a fancy hat!");
    }

    fn reroll() {
        println!("Rolling again!");
    }
}
