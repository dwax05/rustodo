use std::{
    fs::File,
    io::{BufReader, BufRead, Write, BufWriter},
};

pub struct Todo {
    task: String,
    completed: bool,
}

pub struct TodoList {
    todos: Vec<Todo>,
}

impl Todo {
    pub fn new(todo_task: String) -> Self {
        Self { 
            task: todo_task, 
            completed: false, 
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            todos: Vec::new(),
        }
    }

    pub fn print_todos(&self) {
        for todo in &self.todos {
            println!("{} {}", 
                if todo.completed == true { "DONE" }else{ "TODO" }, 
                todo.task
            )
        }
    }

    pub fn get_todos(&self) -> Vec<String> {
        let mut todos: Vec<String> = Vec::new();
        for todo in &self.todos { 
            let todo = Todo::new(String::from(todo.task.as_str()));
            todos.push(todo.task);
        }
        todos
    }

    pub fn write_to_file(&self) {
        let mut buffer = BufWriter::new(File::create("TODO.txt").expect("Could not create file"));

        for todo in &self.todos {
            let completed: String = if todo.completed == true { String::from("DONE ") }else{ String::from("TODO ") };
            let line: String = completed + todo.task.as_str();

            write!(buffer, "{}\n", line).expect("Could not write to file");
        }
    }

    pub fn get_task_index(&self, todo_text: &str) -> Option<usize> {
        for (index, item) in self.todos.iter().enumerate() {
            if item.task == todo_text {
                return Some(index);
            }
        }
        None
    }

    pub fn add_task(&mut self, todo: &str) {
        let todo = Todo::new(String::from(todo));
        self.todos.push(todo);
    }

    pub fn remove_task(&mut self, index: usize) {
        self.todos.remove(index);
    }

    pub fn read_todos_from_file(&mut self){
        let file = File::open("TODO.txt").expect("Could not read file");

        // Create a BufReader to read lines from the file
        let reader = BufReader::new(file);

        // Iterate over each line in the file
        for line in reader.lines() {
            // Check if line reading was successful
            match line {
                Ok(line_content) => {
                    // Do something with the line content
                    let (status, task) = line_content.as_str().split_at(5);
                    let mut todo: Todo = Todo::new(String::from(task));
                    if status == "DONE " {
                        todo.complete();
                    }
                    self.todos.push(todo);
                }
                Err(e) => {
                    eprintln!("Error reading line: {}", e);
                    // Handle the error as needed
                }
            }
        }
    }
}


