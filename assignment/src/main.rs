use std::io;

fn main() {
    println!("Please enter your energy consumption(kwh): ");
    let mut energy = String::new();
    io::stdin().read_line(&mut energy).expect("Failed to read input."); 
    let energy: f64 = energy.trim().parse().expect("Not a valid input");

    if energy >= 100.0 && energy < 200.0 {
        let bill = energy * 25.0;
        println!("Your electricity bill: N{}", bill);
    } else if energy >= 200.0 {
        let bill = energy * 30.0;
        println!("Your electricity bill: N{}", bill);
    } else {
        let bill = energy * 20.0;
        println!("Your electricity bill: N{}", bill);
    }
    
}