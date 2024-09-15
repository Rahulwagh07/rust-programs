//iterators are lazy: they don’t perform any work until they are consumed.

//Methods like sum, collect, and map either consume iterators or produce new ones, 
//allowing flexible data processing.

//Closures used with iterator adaptors can capture variables from their surrounding environment,
// enabling more complex filtering and transformation.

pub fn main() {
  // Create a vector of integers.
  let v1 = vec![1, 2, 3];

  // Create an iterator over the vector.
  let v1_iter = v1.iter();

  // Using the iterator in a for loop to print each value.
  println!("Using iterator in a for loop:");
  for val in v1_iter {
      println!("Got: {val}");
  }
  // Note: The iterator is consumed by the for loop, so we can't use v1_iter again directly.

  // Recreate the iterator since the previous one is exhausted.
  let mut v1_iter = v1.iter();

  // Demonstrate using the next() method to get values from the iterator.
  println!("\nUsing next() method:");
  assert_eq!(v1_iter.next(), Some(&1));
  assert_eq!(v1_iter.next(), Some(&2));
  assert_eq!(v1_iter.next(), Some(&3));
  assert_eq!(v1_iter.next(), None); // No more items left.

  // Attempt to sum the values using the iterator. Note: This will fail because the iterator is exhausted.
  // let total: i32 = v1_iter.sum(); // This line will not work as v1_iter is already used up.

  // Create a new iterator, map each value to its incremented value, and collect results into a vector.
  let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

  println!("\nAfter mapping and collecting:");
  println!("Original vector: {:?}", v1);
  println!("Mapped vector: {:?}", v2);


  let shoes = vec![
      Shoe {
          size: 10,
          style: String::from("sneaker"),
      },
      Shoe {
          size: 13,
          style: String::from("sandal"),
      },
      Shoe {
          size: 10,
          style: String::from("boot"),
      },
  ];

  let size_to_filter = 10;
  let filtered_shoes = shoes_in_size(shoes, size_to_filter);

  println!("\nFiltered shoes of size {}:", size_to_filter);
  for shoe in filtered_shoes {
      println!("{:?}", shoe);
  }
}

// The Iterator trait requires the implementation of the next method,
// which returns the next item or None if the iteration is complete.
// Item is an associated type that represents the type of items the iterator yields.
pub trait Iterator {
  type Item;
  fn next(&mut self) -> Option<Self::Item>;
}

// Define the Shoe struct with size and style.
#[derive(PartialEq, Debug)]
struct Shoe {
  size: u32,
  style: String,
}

// Function to filter shoes by size and return a vector of matching shoes.
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
  shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}