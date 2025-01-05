use std::io;

fn main() {
    println!("Palindrome Checker!");
    println!("Enter a string to check if it is a polindrome:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let cleaned_input = cleaned_string(&input);

    if cleaned_input.is_empty() {
        println!("Please enter a valid non-empty string");
        return;
    }

    if is_polindrome(&cleaned_input) {
        println!("The '{}' is a polindrome!", input.trim());
    } else {
        println!("The '{}' is not a polindrome!", input.trim());
    }
}

fn cleaned_string(input: &str) -> String {
    input
        .chars() // iterate over each character
        .filter(|c| c.is_alphanumeric()) // keep only letters and numbers
        .map(|c| c.to_lowercase().to_string()) // conert to lowercase
        .collect::<String>() // collect into new string
}

// check if cleaned string is polindrome
fn is_polindrome(input: &str) -> bool {
    input == input.chars().rev().collect::<String>()
}
