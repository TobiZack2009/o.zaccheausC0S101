/*
The PAU Café needs a talented problem solver to help them with an Ordering System
complete with discount calculators and they have approached you to assist them.
The Café sells the following:
Code  Item     Price (₦)
T     Tea      800
C     Coffee   1,200
S     Sandwich 2,000
J     Juice    1,500
Program tasks:
1. Display the café menu.
2. Ask the customer for item code and quantity.
3. Compute total cost.
4. If total ≥ ₦5,000, apply a 5% discount.
5. Display the final amount.
6. Allow multiple entries until the user types “exit”.
*/

use std::io;

fn main() {
 let mut orders:Vec<(&str,u32)>=vec![];
 let mut total=0;
    println!(
"Welcome to the PAU Café ordering system!\nWe sell the following:
Code  Item     Price (₦)
T     Tea      800
C     Coffee   1,200
S     Sandwich 2,000
J     Juice    1,500\nTo order an item, type a code and quantity!\nYou can type 'exit' at any time to display the final amount.");
    
    'orders_input: loop{
    println!("Enter a code, then a number!");
    orders.push((match input_code() {
        
        "T"=>"T","C"=>"C","S"=>"S","J"=>"J",_ => break 'orders_input,
    },input_quantity())}


}
fn input_code()->&'static str{
    let mut code_input=String::new();
    io::stdin().read_line(&mut code_input);
    return match code_input.trim() {
        "T"=>"T",
        "exit"=>"exit",
        "C"=>"C",
        "S"=>"S",
        "J"=>"J",
        _=>{
            println!("Enter a valid code!");
            input_code()
}
    }
}
fn input_quantity()->u32{
let mut num_input=String::new();
io::stdin().read_line(&mut num_input).expect("");
if (num_input=="exit"){return 80000; };
return match num_input.trim().parse() {
    Ok(val)=>val,
    Err(_)=>{
        println!("Enter a valid number!");
        input_quantity()
    }
}
}