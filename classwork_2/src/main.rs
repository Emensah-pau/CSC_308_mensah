use std::io;

fn main() {
    println!("Please enter your bill: N");
    let mut bill = String::new();
    io::stdin().read_line(&mut bill).expect("Failed to read input."); 
    let bill: f64 = bill.trim().parse().expect("Please enter a valid number!");

    if bill >= 5000.0 && bill < 10000.0 {
        let discount = bill * 0.9;
        println!("Your after discount added: N{}", discount);
    } else if bill >= 10000.0 {
        let discount = bill * 0.85;
        println!("Your after discount added: N{}", discount);
    } else {
        println!("No discount added");
    }
    
}