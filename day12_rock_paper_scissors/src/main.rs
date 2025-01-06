use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to Rock-Paper-Scissors!");
    println!("Instruction: Enter 'rock', 'paper', or 'scissors'. Type 'quit' to exit");

    loop {
        println!("\n make your choice:");

        let user_choice = get_user_choice();
        if user_choice == "quit" {
            println!("Thanks for playing! Goodbye!");
            break;
        }

        let computer_choice = get_computer_choice();
        println!("Computer chose: {}", computer_choice);

        match determine_winner(&user_choice, &computer_choice) {
            GameResult::Win => println!("You win"),
            GameResult::Lose => println!("You lose!"),
            GameResult::Draw => println!("It's a draw!"),
        }
    }
}

// read and validate user input
fn get_user_choice() -> String {
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("failed to read input");

    let choice = choice.trim().to_lowercase();
    match choice.as_str() {
        "rock" | "paper" | "scissors" | "quit" => choice,
        _ => {
            println!("Invalid choice, Please enter 'rock', 'paper', 'scissors', or 'quit'.");
            get_user_choice()
        }
    }
}

// Generates a random choice for the computer
fn get_computer_choice() -> String {
    let choice = ["rock", "paper", "scissors"];
    let index = rand::thread_rng().gen_range(0..choice.len());
    choice[index].to_string()
}

// Enum to represent game outcomes
enum GameResult {
    Win,
    Lose,
    Draw,
}

// Determince the game outcome
fn determine_winner(user: &str, computer: &str) -> GameResult {
    match (user, computer) {
        ("rock", "scissors") => GameResult::Win,
        ("paper", "rock") => GameResult::Win,
        ("scissors", "paper") => GameResult::Win,
        (a, b) if a == b => GameResult::Draw,
        _ => GameResult::Lose,
    }
}
