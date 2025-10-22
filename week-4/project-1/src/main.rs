//Written by Oluwatobiloba Zaccheaus.
fn main() {
    let mut start_message="Enter a quadratic equation in the form ax²+bx+c=0.";
    //Start message is a variable because it can change to an error message if the solution is not real.
 loop{   
    clear_screen();
    println!("{}",start_message);
    let values=get_values(); //Get our values.
    let a:f64=values[0];
    let b:f64=values[1];
    let c:f64=values[2];

    let discriminant=((b*b)-(4.0*a*c));
    if discriminant<0.0 {
        start_message="Invalid values, discriminant is less than 0. This solver can only handle real solutions.";   
        continue;
    };
    
    let x1=(-b+discriminant.powf(0.5))/(2.0*a);
    let x2=(-b-discriminant.powf(0.5))/(2.0*a);
    
    println!("x is equal to {} and {}.",x1,x2);
    break;
}}


//A function to input values as numbers, with error handling.
fn input_number(mut message: &str)-> f64 {
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

fn get_values()-> [f64; 3] {
    //Function to get a, b and c.
    use std::io;
    return [input_number("Enter term a."),input_number("Enter term b."),input_number("Enter term c.")];
}

fn clear_screen(){ //Clears the screen.
    print!("\x1B[2J\x1B[1;1H");
}