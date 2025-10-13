use std::io;
fn main(){
   println!(" 1, Celsius to Farenheit ");
   println!(" 2, Farenheit to Celcius ");
   println!(" 1 or 2 ");
   
   let mut choice=String::new();
   io::stdin().read_line(&mut choice).unwrap();
   let choice = choice.trim();

   println!("Enter a temperature value");
   let mut temp=String::new();
   io::stdin().read_line(&mut temp).unwrap();

     let temp: f64 = temp.trim().parse().unwrap();
   if choice == "1" {
    let farenheit = (temp * 9.0/5.0) + 32.0;
    println!("Temperature: {}°C", temp);
    println!("Converted: {}°F", farenheit);
   }
   else if choice == "2"{
    let celcius = (temp - 32.0) * 5.0 / 9.0;
    println!("Temperature: {}°F", temp);
    println!("Converted: {}°C", celcius);
   }
   else{
    println!("Invalid choice")
   }
}

