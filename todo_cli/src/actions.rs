use crate::errors::AppError;

#[derive(Debug)]
pub enum Action {
    Add(String),
    Complete(usize),
    List,
    Remove(usize),
    Clear,
}

pub fn parse_args(args: Vec<String>) -> Result<Action, AppError> {
    if args.len() < 2 {
        return Err(AppError::ParseError(
            "Usage: todo-cli [add|list|complete|remove|clear|help] [description|id]".to_string(),
        ));
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                return Err(AppError::ParseError(
                    "Usage: todo-cli add \"task description\"".to_string(),
                ));
            }
            Ok(Action::Add(args[2].clone()))
        }
        "complete" => {
            if args.len() < 3 {
                return Err(AppError::ParseError(
                    "Usage: todo-cli complete <task-id>".to_string(),
                ));
            }
            let id = args[2]
                .parse()
                .map_err(|_| AppError::ParseError("Invalid task ID format".to_string()))?;
            Ok(Action::Complete(id))
        }
        "remove" => {
            if args.len() < 3 {
                return Err(AppError::ParseError(
                    "Usage: todo-cli remove <task-id>".to_string(),
                ));
            }
            let id = args[2]
                .parse()
                .map_err(|_| AppError::ParseError("Invalid task ID format".to_string()))?;
            Ok(Action::Remove(id))
        }
        "list" => Ok(Action::List),
        "clear" => Ok(Action::Clear),
        "help" => {
            println!("Usage: todo-cli [add|list|complete|remove|clear|help] [description|id]");
            std::process::exit(0);
        }
        _ => Err(AppError::ParseError(
            "Invalid command. Available commands: add, list, complete, remove, clear, help"
                .to_string(),
        )),
    }
}
