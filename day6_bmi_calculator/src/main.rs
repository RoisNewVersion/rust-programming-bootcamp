// BMI = weight(kgs)/height2(m2)
// BMI < 18.5 UW
// BMI 18.5 - 24.9 N
// BMI 25-29.9 OW
// BMI >= 30 Obesity

use std::io;

fn main() {
    println!("BMI calculator");

    println!("Please enter your weight in kilograms (Kg):");
    let weight = match get_input_as_f64() {
        Some(val) => val,
        None => {
            println!("Invalid input for weight. Please enter valid number!");
            return;
        }
    };

    println!("Please enter your height in meters (m):");
    let height = match get_input_as_f64() {
        Some(val) => val,
        None => {
            println!("Invalid input for height. Please enter valid number!");
            return;
        }
    };

    if height == 0.0 {
        println!("Height cannot be zero.");
        return;
    }

    let bmi = calculate_bmi(weight, height);
    println!("Your BMI is: {:.2}", bmi);

    let category = clasify_bmi(bmi);
    println!("BMI category: {}", category);
}

fn get_input_as_f64() -> Option<f64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read value");
    match input.trim().parse::<f64>() {
        Ok(result) => Some(result),
        Err(_) => None,
    }
}

fn calculate_bmi(weight: f64, height: f64) -> f64 {
    weight / (height * height)
}

fn clasify_bmi(bmi: f64) -> &'static str {
    if bmi < 18.5 {
        "Underwight"
    } else if bmi >= 18.5 && bmi <= 24.9 {
        "Normal weight"
    } else if bmi >= 25.0 && bmi <= 29.9 {
        "Overweight"
    } else {
        "Obesity"
    }
}
