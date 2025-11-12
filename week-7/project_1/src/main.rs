


fn main() {
  print_welcome();
loop{
    match input_number("Enter a valid number.") {
        0f64=>{
            break;
        }
        1f64=>{
             calc_trapezium();
        }
        2f64=>{
            calc_rhombus();
        }
        3f64=>{
            calc_parallelogram();
        }
        4f64=>{
            calc_cube();
        }
        5f64=>{
            calc_cylinder();
        }
        _=>{
            println!("Enter a valid number!")
        }
    }
}
println!("Thank you!")
}

fn input_number(fallback:&str)->f64{
    use std::io;
    let mut input_str=String::new();
    loop{
        io::stdin().read_line(&mut input_str).expect("There was an error");

    return  match input_str.trim().parse() {
            Ok(val)=>val,
            Err(_)=>{
                input_str.clear();
                println!("{fallback}");
                continue;
            }
        };


    };
    

}

fn print_welcome(){
    print!("\x1B[2J");
    println!("Hello! This is a Rust program that performs the following calculations:
\n1. Area of Trapezium\n2. Area of the Rhombus\n3. Area of Parallelogram\n4. Area of Cube\n5. Volume of Cylinder\n\nEnter a number to perform a calculation, or 0 to exit.");
}

fn calc_trapezium(){
    println!("Enter the height of your trapezium.");
    let h=input_number("Enter a valid number");
    println!("Enter the length of the first base of your trapezium.");
    let b1=input_number("Enter a valid number");
    println!("Enter the length of the second base of your trapezium.");
    let b2=input_number("Enter a valid number");
    let area=0.5*(b1+b2)*h;
    println!("The area of your trapezium is: {}.",area);
    print_welcome();
}

fn calc_rhombus(){
    println!("Enter the size of the first diagonal of the rhombus.");
    let d1=input_number("Enter a valid number");
    println!("Enter the size of the second diagonal of the rhombus.");
    let d2=input_number("Enter a valid number");
    println!("The area of your rhombus is: {}.",0.5*d1*d2);
    
}

fn calc_parallelogram(){
    println!("Enter the size of the base of parallelogram.");
    let b=input_number("Enter a valid number");
    println!("Enter the size of the parallelogram's altitiude..");
    let a=input_number("Enter a valid number");
    println!("The area of your rhombus is: {}.",b*a);
    
}

fn calc_cube(){
    println!("Enter the length of your cube.");
    println!("The surface area of your cube is: {}",6.0*input_number("Enter a valid number.").powf(2.0));
    
}

fn calc_cylinder(){
    println!("Type the radius, press enter, and then type the height of the cylinder.");
    println!("The volume of the cylinder is: {}",22.0/7.0*input_number("Enter a valid number!").powf(2.0)*input_number("Enter a valid number!"));
    
}