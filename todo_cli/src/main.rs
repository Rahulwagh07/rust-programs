mod todo;
mod actions;
mod errors;

use std::env;
use actions::{Action, parse_args};
use todo::TodoList;
use errors::AppError;

fn main() {
    let args: Vec<String> = env::args().collect();
  
    if args.len() > 1 && args[1] == "help" {
        println!("Usage: todo-cli [add|list|complete|remove|clear|help] [description|id]");
        return;
    }

    let mut todo_list = TodoList::load_or_create().unwrap_or_else(|err| {
        eprintln!("Failed to load todo list: {:?}", err);
        TodoList::new()
    });

    let result = match parse_args(args) {
        Ok(action) => match action {
            Action::Add(description) => todo_list.add(description),
            Action::Complete(id) => todo_list.complete(id),
            Action::Remove(id) => todo_list.remove(id),
            Action::List => {
                todo_list.list();
                Ok(())
            },
            Action::Clear => todo_list.clear(),
        },
        Err(err) => {
            eprintln!("Error: {}", match err {
                AppError::ParseError(msg) => msg,
                _ => "An unexpected error occurred".to_string(),
            });
            std::process::exit(1);
        }
    };

    if let Err(err) = result {
      match err {
          AppError::IoError(e) => eprintln!("I/O Error: {:?}", e),
          AppError::JsonError(e) => eprintln!("JSON Error: {:?}", e),
          AppError::ParseError(msg) => eprintln!("Error: {}", msg),
      }
      std::process::exit(1);
  }
}
