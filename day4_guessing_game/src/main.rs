use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to guessing game!");
    println!("I'm thinking of a number between 1 and 100. Can you guess it?");
    
    // Generate a random nummber between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");

        let guess: u32 = match guess.trim().parse(){
            Ok(result) => result,
            Err(_) => {
                println!("Please enter valid number!");
                continue;
            }
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Try again"),
            Ordering::Greater => println!("Too big! Try again"),
            Ordering::Equal => {
                println!("Congratulation! You guessed the number!");
                break;
            }
        }

    }
}
