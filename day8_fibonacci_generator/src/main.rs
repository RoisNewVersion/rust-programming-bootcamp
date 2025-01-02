use std::io;

fn main() {
    println!("Fibonacci Sequence Generator!");
    println!("Enter the number of terms you want to generate:");

    let num_terms = match get_input_as_u32() {
        Some(val) => val,
        None => {
            println!("Invalid input. Please enter a positive integer");
            return;
        }
    };

    if num_terms == 0 {
        println!("Number of term must be greater than zero!");
        return;
    }

    let sequence = generate_fibonacci(num_terms);
    println!("Fibonacci Sequence ({} terms): {:?}", num_terms, sequence);
}

// Reads user input and attemps to parse it as u32
fn get_input_as_u32() -> Option<u32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    match input.trim().parse::<u32>() {
        Ok(val) => Some(val),
        Err(_) => None,
    }
}

// Generaters a Fibonacci sequence up to 'n' terms
fn generate_fibonacci(n: u32) -> Vec<u64> {
    let mut sequence = Vec::new();
    if n >= 1 {
        sequence.push(0); // first term
    }
    if n >= 2 {
        sequence.push(1); // second term
    }

    for i in 2..n {
        let next = sequence[i as usize - 1] + sequence[i as usize -2];
        sequence.push(next);
    }

    sequence
}