use std::io::Write;

struct Task {
    description: String,
    done: bool,
}

impl Task {
    fn new(description: &str) -> Task {
        Task {
            description: description.to_string(),
            done: false,
        }
    }
}

struct TodoApp {
    tasks: Vec<Task>,
}

impl TodoApp {
    fn new() -> TodoApp {
        TodoApp { tasks: Vec::new() }
    }

    fn add_new_task(&mut self, description: &str) {
        let task: Task = Task::new(description);
        self.tasks.push(task);
    }

    fn mark_task_as_done(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.done = true;
        } else {
            println!("Invalid task index!");
        }
    }

    fn show_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            let status = if task.done { "[X]" } else { "[]" };
            println!("{}: {} {}", index + 1, status, task.description);
        }
    }
}

fn get_string_input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().ok();
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");
    input.trim().to_string()
}

fn get_numeric_input(prompt: &str) -> Option<usize> {
    print!("{}", prompt);
    std::io::stdout().flush().ok();
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");
    match input.trim().parse() {
        Ok(value) => Some(value),
        Err(_) => {
            println!("Invalid input, please enter a valid number.");
            None
        }
    }
}

fn main() {
    let mut todo_app = TodoApp::new();

    loop {
        println!("\n--- Todo App ---");
        println!("1. Add new task");
        println!("2. Mark task as done");
        println!("3. Show tasks");
        println!("4. Exit");

        let choice = match get_numeric_input("Enter your choice: ") {
            Some(value) => value,
            None => continue,
        };

        match choice {
            1 => {
                let description = get_string_input("Enter task description: ");
                todo_app.add_new_task(&description);
            }
            2 => {
                let index = match get_numeric_input("Enter the task index to mark as done: ") {
                    Some(value) => value,
                    None => continue,
                };
                if index > 0 {
                    todo_app.mark_task_as_done(index - 1);
                } else {
                    println!("Task index must be greater than 0!");
                }
            }
            3 => todo_app.show_tasks(),
            4 => {
                println!("Exiting Todo App. Goodbye!");
                break;
            }
            _ => println!("Invalid option, please enter a valid number."),
        }
    }
}
