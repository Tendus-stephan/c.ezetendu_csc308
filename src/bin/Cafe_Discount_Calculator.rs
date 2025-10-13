use std::io;
fn main(){
    println!("Enter the total bill amount:");

    let mut bill_amount=String::new();
    io::stdin().read_line(&mut bill_amount).unwrap();
    let bill_amount: f64 = bill_amount.trim().parse().unwrap();

    if bill_amount >= 5000.0 && bill_amount < 10000.0{
        let discount = 10.0;
        let final_amount =bill_amount / discount;
        println!("Original Bill: ₦{}",bill_amount);
        println!("Discount Applied: {}%",discount);
        println!("Final Bill: ₦{}",final_amount);
    }
    else if bill_amount >= 10000.0{
        let discount = 15.0;
        let final_amount =bill_amount / discount;
        println!("Original Bill: ₦{}",bill_amount);
        println!("Discount Applied: {}%",discount);
        println!("Final Bill: ₦{}",final_amount);
    }
    else{
        println!("No discount applied. Final Bill: ₦{}",bill_amount)
    }

}