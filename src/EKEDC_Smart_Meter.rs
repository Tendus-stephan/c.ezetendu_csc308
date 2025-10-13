use std::io;

fn main() {
    println!("Enter your energy consumption in kWh:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Convert input to f64 without checking
    let consumption: f64 = input.trim().parse().unwrap();

    // Calculate bill
    let bill = if consumption <= 100.0 {
        consumption * 20.0
    } else if consumption <= 200.0 {
        consumption * 25.0
    } else {
        consumption * 30.0
    };

    println!("Your total electricity bill is: ₦{:.2}", bill);
}
