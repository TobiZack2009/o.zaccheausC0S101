/*
A microfinance bank has employed you to work on their student loan repayment
estimator. They explain that a student borrows an amount, P, at an annual interest rate,
R (%) for T years. The amount due after T years is:
A = P × (1 + R/100) ^T
You are tasked to write a Rust program that meets these requirements:
1. 2. 3. Accept P, R, and T via keyboard input.
Computes both A and the local interest (A - P).
Asks if the user wants to calculate for another borrower. Continue if “y”, end if
“n”.
4. Use appropriate type casting and loop.




*/
use std::io;
//Written by Oluwatobiloba Zaccheaus
fn main() {
    println!("Hello! This is a student loan repayment calculator.");
    println!("The amount to be repaid will be computed using this formula:\n A = P × (1 + R/100) ^T\nWhere A is amount to be repaid, P is amount borrowed, and R is the annual interest rate, and T is the amount of years.");
'main: loop{ //Labelled the loop to allow it to be continured by the match statement.


    
    //Ask the user to input their values.
    println!("Enter your P: ");
    let mut p=input_number();
    println!("Enter your R: ");
    let mut r=input_number();
    println!("Enter your T: ");
    let mut t=input_number();

    //Caluculate the amount a and interest i.
    let mut a=p*(1.0+r/100.0).powf(t);
    let mut i=a-p; //Interest is principal minus amount.

    //Finally show the output.
    println!("The interest is ₦{:} and the amount to be repaid is ₦{:}",i,a );
    println!("Would you like to calculate for another student?");
    'yninput: loop{
    let mut user_input=String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    match user_input.trim() {
        "y"=> continue 'main,
        "n"=> break 'main,
        _=>{
            println!("Enter y or n");
            continue 'yninput;
        },

    }}

}}

fn input_number()->f64{ //A function to input numbera.
    use std::io; //Import io from standard libary.
    let mut user_input=String::new(); //This will hold the user's input.
    io::stdin().read_line(&mut user_input).expect("Failed to read line"); //Read input from stdin.
    return match user_input.trim().parse(){
        Ok(val)=>val, //If the input is a number return it.
        Err(_)=>{
            println!("Invalid number!");
            input_number() //If it isn't ask the user to enter a valid number again.
        }
    }
}