use std::io::{self, Write};
use std::{thread, time::Duration};

fn main() {
    println!("Basic timer tool!");
    println!("Enter the timer duration (format: hours minute seconds)");

    let duration = match get_timer_input() {
        Some(dur) => dur,
        None => {
            println!(
                "Invalid input, please enter numbers only (e.g., 0 1 30 for 1 minute 30 seconds)"
            );
            return;
        }
    };

    println!(
        "Timer set for: {} hours, {} minutes, {} seconds",
        duration.0, duration.1, duration.2
    );

    start_timer(duration.0, duration.1, duration.2);
    println!("Time's up");
}

// parse user input for hours, minute, and seconds
fn get_timer_input() -> Option<(u64, u64, u64)> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 3 {
        return None;
    }

    let hours = parts[0].parse::<u64>().ok()?;
    let minutes = parts[1].parse::<u64>().ok()?;
    let seconds = parts[2].parse::<u64>().ok()?;

    Some((hours, minutes, seconds))
}

// Start the timer and display countdown
fn start_timer(hours: u64, minute: u64, seconds: u64) {
    let total_seconds = hours * 3600 + minute * 60 + seconds;
    for i in (1..=total_seconds).rev() {
        let hrs = i / 3600;
        let mins = (i % 3600) / 60;
        let secs = i % 60;

        print!("\r Time Remaining: {:02}:{:02}:{:02}", hrs, mins, secs);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    println!(""); // move to a new line after the countdown ends
}
