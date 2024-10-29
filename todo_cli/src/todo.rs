use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    id: usize,
    description: String,
    status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    todos: Vec<Todo>,
    next_id: usize,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    pub fn load_or_create() -> Result<Self, AppError> {
        let path = TodoList::get_data_file_path();
        if path.exists() {
            let file = fs::File::open(path)?;
            Ok(serde_json::from_reader(file)?)
        } else {
            Ok(TodoList::new())
        }
    }

    fn get_data_file_path() -> PathBuf {
        let mut path = PathBuf::from(".");
        path.push("todo_list.json");
        path
    }

    pub fn save(&self) -> Result<(), AppError> {
        let path = TodoList::get_data_file_path();
        let file = fs::File::create(path)?;
        serde_json::to_writer_pretty(file, &self)?;
        Ok(())
    }

    pub fn add(&mut self, description: String) -> Result<(), AppError> {
        self.todos.push(Todo {
            id: self.next_id,
            description,
            status: false,
        });
        self.next_id += 1;
        self.save()?;
        println!("Task added successfully!");
        Ok(())
    }

    pub fn complete(&mut self, id: usize) -> Result<(), AppError> {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.status = true;
            self.save()?;
            println!("Task {} marked as complete!", id);
        } else {
            println!("Task {} not found!", id);
        }
        Ok(())
    }

    pub fn remove(&mut self, id: usize) -> Result<(), AppError> {
        if let Some(pos) = self.todos.iter().position(|t| t.id == id) {
            self.todos.remove(pos);
            self.save()?;
            println!("Task {} removed!", id);
        } else {
            println!("Task {} not found!", id);
        }
        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), AppError> {
        self.todos.clear();
        self.next_id = 1;
        self.save()?;
        println!("All tasks cleared!");
        Ok(())
    }

    pub fn list(&self) {
        if self.todos.is_empty() {
            println!("No tasks found!");
            return;
        }
        
        println!("\nTODO List:");
        println!("----------");
        for todo in &self.todos {
            println!(
                "{}: [{}] {}",
                todo.id,
                if todo.status { "✓" } else { " " },
                todo.description
            );
        }
        println!();
    }
}
