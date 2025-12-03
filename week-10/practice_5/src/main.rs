fn main() {
    let x=vec![100,200,300]; //A list of numbers.
    borrow_vector(&x); //Passed reference.

    println!("Printing the value from main() x[0]={}",x[0]);
    println!("**************************");
}

fn borrow_vector(z:&Vec<i32>){
    println!("**************************");
    println!("Inside print_vector function {:?} \n",z);
    println!("--------------------------");

}
