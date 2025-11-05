/*
You have been employed by an electricity distribution company (EKDC) to write a Rust
program that calculates a customer’s monthly electricity bill based on their
consumption in kilowatt-hours (kWh):
Units Consumed (kWh) Rate per Unit (₦)
0–100                20
101-300              35
301 and above        50
Requirements:
● Prompt the user to enter customer name and units consumed.
● Use if...else to decide the rate and compute total.
● If units > 500, add ₦5,000.00
● Display the customer’s name, units, rate and total bill neatly.
*/
use std::io;
fn main() {
    //Read the user name
    println!("Enter your name:");
    let mut user_name=String::new();
    io::stdin().read_line(&mut user_name).expect("Failed to read line.");

    //Read the number of units bought.
    println!("Input the number of units you have bought from EKDC:");
    let mut unit_number:f64=0.0;
    let mut rate:f64=0.0;
    //Get the rate from the stdin.  
    loop{  
        let mut user_input=String::new();
        io::stdin().read_line(&mut user_input);
        match user_input.trim().parse(){
            Ok(val)=> {
                unit_number=val;
                break;
            }
            Err(_)=>{
                println!("Enter a valid positive nonzero number!");
                continue;
            }}};

    let mut total:f64=0.0; //The total amount to be paid.
    unit_number=unit_number.abs(); //Find absolute value in case user put a neagative value.
    //Depending on number of units  bought, calculate the rate. 
    if unit_number<100.0 && unit_number>=0.0 {
        rate=20.0
    }
    else if unit_number>100.0 && unit_number<300.0 {
        rate=35.0;
    }
    else if unit_number>300.0 {
        rate=50.0;
    };
    //If more than 500 units were added, add 5000 to total cost.
    if unit_number>500.0 {
        total=total+5000.0;
    }

//Calculate the final total.
total=unit_number*rate;
println!("Receipt for Customer {}\n\nUnits Consumed: {}\nAmount to be paid: ₦{}\nCustomer Name: {}",user_name.trim(),unit_number,total,user_name.trim() );
}
