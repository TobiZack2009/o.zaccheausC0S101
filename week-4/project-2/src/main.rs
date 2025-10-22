//Written by Oluwatobiloba Zaccheaus
fn main() {
    loop{
    clear_screen();
    println!("Hello! This is a program to determine your annual incentive.");
    println!("Please answer honestly.");
    let age:u8=input_number("Enter your age.") as u8;
    let is_experienced=input_yesno("Do you have experience?");

    if is_experienced&& age>=40 {
        println!("Congratulations! Your bonus is ₦1,560,000!")
    }
    if is_experienced&& age>=30 && age<40 {
        println!("Congratulations! Your bonus is ₦1,480,000!")
    }
    if !is_experienced {
        println!("Congratulations! Your bonus is ₦100,000!")
    };
   break;

}}

//A function to input values as numbers, with error handling.
fn input_number(message: &str)-> f64 {

    use std::io;
    println!("{}",message.to_string());
    loop {
        let mut data =String::from("");
        io::stdin()
            .read_line(&mut data)
            .expect("Failed to read line");        
     return match data.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter a valid number.");
            data.clear(); //Make sure the input data is cleared.
            continue;
        },
     }
    }
}

fn input_yesno(message: &str)-> bool {
//A function to input either Yes or No
    use std::io;
    println!("{}",message.to_string());
    loop {
        let mut data =String::from("");
        io::stdin()
            .read_line(&mut data)
            .expect("Failed to read line");        
     return match data.trim() {
        "Yes" => true,
        "No" => false,
        _ => {
            println!("Enter either a 'Yes' or 'No'!");
            continue;
        },
     }
    }
}

fn clear_screen(){ //Clears the screen.
    print!("\x1B[2J\x1B[1;1H");
}