//Traits -> used to define shared behavior across different types. 
//They specify a set of methods that a type must implement.

//syntax
pub trait TraitName {
  fn method_name(&self) -> ReturnType;
}

pub trait Summary {
  fn summarize(&self) -> String;
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self location)
  }
}
