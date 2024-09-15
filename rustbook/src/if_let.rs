#[derive(Debug)] 
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn main() {
    //match with Coin enum
    let coin = Coin::Quarter(UsState::Alaska);

    match coin {
        Coin::Penny => println!("Penny!"),
        Coin::Nickel => println!("Nickel!"),
        Coin::Dime => println!("Dime!"),
        Coin::Quarter(state) => println!("State quarter from {:?}", state),
    }

    //Option<T> with match
    let x: Option<i32> = Some(5);
    let result = match x {
        Some(value) => value + 1,
        None => 0,
    };
    println!("Result is {}", result);

    //if let with Option<T>
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    //handling only some patterns with if let
    let mut count = 0;
    let another_coin = Coin::Dime;

    if let Coin::Quarter(state) = another_coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
    println!("Number of non-quarter coins: {}", count);

}
