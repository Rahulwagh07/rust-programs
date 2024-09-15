// Define the IpAddrKind enum
enum IpAddrKind {
  V4,
  V6,
}

// Define a struct using IpAddrKind
struct IpAddr {
  kind: IpAddrKind,
  address: String,
}

// Define an enum with associated data
enum IpAddrWithData {
  V4(String),
  V6(String),
}

// Define an enum with different data types
enum IpAddrWithDifferentData {
  V4(u8, u8, u8, u8),
  V6(String),
}

// Define a Message enum with various types of data
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

// Implement a method on the Message enum
impl Message {
  fn call(&self) {
      // Example method body
      println!("Message received!");
  }
}


pub fn main() {
  // Create instances of IpAddrKind enum
  let _four = IpAddrKind::V4;
  let _six = IpAddrKind::V6;

  // Create instances of IpAddr struct
  let _home = IpAddr {
      kind: IpAddrKind::V4,
      address: String::from("127.0.0.1"),
  };

  let _loopback = IpAddr {
      kind: IpAddrKind::V6,
      address: String::from("::1"),
  };

  // Create instances of IpAddrWithData enum
  let _home_with_data = IpAddrWithData::V4(String::from("127.0.0.1"));
  let _loopback_with_data = IpAddrWithData::V6(String::from("::1"));

  // Create instances of IpAddrWithDifferentData enum
  let _home_with_diff_data = IpAddrWithDifferentData::V4(127, 0, 0, 1);
  let _loopback_with_diff_data = IpAddrWithDifferentData::V6(String::from("::1"));

  // Create instances of Message enum and call the method
  let m = Message::Write(String::from("hello"));
  m.call();

  let m_move = Message::Move { x: 10, y: 20 };
  m_move.call();

  // Using Option<T> enum
  let _some_number = Some(5);
  let _some_char = Some('e');
  let _absent_number: Option<i32> = None;

  // Match example with Option
  let x: i8 = 5;
  let y: Option<i8> = Some(5);

  match y {
      Some(value) => println!("Sum is {}", x + value),
      None => println!("No value"),
  }
}
