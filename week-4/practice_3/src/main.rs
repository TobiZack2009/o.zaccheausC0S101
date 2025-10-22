use std::io;
fn main() {
    // Rust program to calculate triangle area given base and height
    let mut input1=String::new();
    let mut input2=String::new();
    let mut base:f64=0.0;
    let mut height:f64=0.0;
    let mut area:f64=0.0;
    println!("Enter base: ");

    io::stdin().read_line(&mut input1).expect("Invalid string.");

    println!("Enter height: ");
    io::stdin().read_line(&mut input2).expect("Invalid string.");

    base=input1.trim().parse().expect("Enter an number.");
    height=input2.trim().parse().expect("Enter an number.");
    area=base*height*0.5;

    println!("The area is {area}.");
}
