use std::io;

fn main() {
    println!("Please enter the temperature (e.g., 100):");
    let mut temperature = String::new();
    io::stdin().read_line(&mut temperature).expect("Failed to read input."); 
    
    let temperature: f64 = temperature.trim().parse().expect("Please enter a valid number!");
    println!("Is that Celsius (C) or Fahrenheit (F)? Enter C or F:");
    let mut unit_input = String::new();
    io::stdin().read_line(&mut unit_input).expect("Failed to read input.");

    if unit_input.trim().to_uppercase() == "C" {
        let fahrenheit = (temperature * 9.0 / 5.0) + 32.0;
        println!("Temperature: {}°C", temperature);
        println!("Converted: {}°F", fahrenheit); 
    } else if unit_input.trim().to_uppercase() == "F" {
        let celsius = (temperature - 32.0) * 5.0 / 9.0;
        println!("Temperature: {}°F", temperature);
        println!("Converted: {}°C", celsius);
    } else {
        println!("Sorry, I don't recognize that unit. Please run the program again and enter C or F.");
    }
}