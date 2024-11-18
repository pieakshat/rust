use std::io::{self, Write};

struct Expense {
    description: String,
    amount: u32,
}

impl Expense {
    fn new(description: &str, amount: u32) -> Expense {
        Expense {
            description: description.to_string(),
            amount,
        }
    }
}

struct Budget {
    total_budget: u32,
    expenses: Vec<Expense>,
}

impl Budget {
    fn new(total_budget: u32) -> Budget {
        Budget {
            total_budget,
            expenses: Vec::new(),
        }
    }

    fn add_expense(&mut self, description: &str, amount: u32) {
        if self.remaining_budget() >= amount {
            self.expenses.push(Expense::new(description, amount));
            println!("Expense added: {} - {}", description, amount);
        } else {
            println!("Insufficient budget to add this expense.");
        }
    }

    fn remaining_budget(&self) -> u32 {
        let total_expenses: u32 = self.expenses.iter().map(|e| e.amount).sum();
        self.total_budget.saturating_sub(total_expenses)
    }

    fn list_expenses(&self) {
        println!("Expenses:");
        if self.expenses.is_empty() {
            println!("No expenses recorded.");
        } else {
            for (i, expense) in self.expenses.iter().enumerate() {
                println!("{}. {} - {}", i + 1, expense.description, expense.amount);
            }
        }
    }
}

fn get_string_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn get_numeric_input(prompt: &str) -> Option<u32> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse().ok()
}

fn main() {
    let total_budget = loop {
        if let Some(value) = get_numeric_input("Enter total budget: ") {
            break value;
        } else {
            println!("Invalid input. Please enter a valid number.");
        }
    };

    let mut budget = Budget::new(total_budget);

    loop {
        println!("\n--- Budget App ---");
        println!("1. Add expense");
        println!("2. View remaining budget");
        println!("3. List expenses");
        println!("4. Exit");

        let choice = match get_numeric_input("Enter your choice: ") {
            Some(value) => value,
            None => {
                println!("Invalid input. Please enter a number between 1 and 4.");
                continue;
            }
        };

        match choice {
            1 => {
                let description = get_string_input("Enter expense description: ");
                let amount = match get_numeric_input("Enter expense amount: ") {
                    Some(value) => value,
                    None => {
                        println!("Invalid input. Expense not added.");
                        continue;
                    }
                };
                budget.add_expense(&description, amount);
            }
            2 => {
                println!("Remaining budget: {}", budget.remaining_budget());
            }
            3 => {
                budget.list_expenses();
            }
            4 => {
                println!("Exiting the Budget App. Goodbye!");
                break;
            }
            _ => {
                println!("Invalid option. Please choose a number between 1 and 4.");
            }
        }
    }
}
