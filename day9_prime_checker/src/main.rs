use std::io;

fn main() {
    println!("Prime number checker!");
    println!("Enter a positive integer to check if it is prime:");

    let number = match get_input_as_u32() {
        Some(val) => val,
        None => {
            println!("Invalid input. Please enter a positive integer");
            return;
        }
    };

    if number <= 1 {
        println!("The number must be greater than 1.");
        return;
    }

    if is_prime(number) {
        println!("{} is a prime number.", number);
    } else {
        println!("{} is not a prime number.", number);
    }
}

// Read user input and attemps to parse it as u32
fn get_input_as_u32() -> Option<u32> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    match input.trim().parse::<u32>() {
        Ok(result) => Some(result),
        Err(_) => None,
    }
}

// Check if a number is prime
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true; // 2 is the only even prime number
    }
    if n % 2 == 0 {
        return false; // Eliminate even number greate than 2
    }

    let limit = (n as f64).sqrt() as u32 + 1;
    for i in 3..limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}
