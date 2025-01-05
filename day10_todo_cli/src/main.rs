use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};

#[derive(Deserialize, Serialize, Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

fn main() {
    let mut tasks: Vec<Task> = load_tasks();
    loop {
        println!("To-Do List Menu:");
        println!("1. Add Task");
        println!("2. View Task");
        println!("3. Mark Task as Complete");
        println!("4. Delete Task");
        println!("5. Exit");

        let choice = get_input("Enter your choice: ");
        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => view_tasks(&tasks),
            "3" => mark_task_complete(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => {
                save_tasks(&tasks);
                println!("Tasks saved. Goodbye");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

// Get user input from CLI
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}

// Load task from the file
fn load_tasks() -> Vec<Task> {
    match fs::read_to_string("tasks.json") {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}

// Save tasks to file
fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    let mut file = File::create("tasks.json").expect("Failed to save tasks");
    file.write_all(json.as_bytes()).expect("Failed to write tasks to file");
}

// Add a task
fn add_task(tasks: &mut Vec<Task>) {
    let description = get_input("Enter task description: ");
    let id = tasks.len() + 1;
    tasks.push(Task {
        id,
        description: description.trim().to_string(),
        completed: false,
    });
    println!("Task added!");
}

// View all tasks
fn view_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks found!");
    } else {
        for task in tasks {
            let status = if task.completed { "v" } else { "x" };
            println!("{} - {}: {}", task.id, status, task.description);
        }
    }
}

// Mark a task as complete
fn mark_task_complete(tasks: &mut Vec<Task>) {
    let id = get_input("Enter task ID to mark as complete: ");
    if let Ok(id) = id.trim().parse::<usize>() {
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            println!("Task marked as complete!");
        } else {
            println!("Task not found.");
        }
    } else {
        println!("Invalid task ID.");
    }
}

// Delete a task
fn delete_task(tasks: &mut Vec<Task>) {
    let id = get_input("Enter task ID to delete: ");
    if let Ok(id) = id.trim().parse::<usize>() {
        if let Some(index) = tasks.iter().position(|t| t.id == id) {
            tasks.remove(index);
            println!("Task deleted!");
        } else {
            println!("Task not found.");
        }
    } else {
        println!("Invalid task ID");
    }
}
