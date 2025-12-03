fn main() {
    //A list of numbers.
    let v=vec![15,25,35,45,55];
    //print_vector(v); //Replace this with a clone
    print_vector(v.clone());
    println!("{}",v[0]); //This will give an error.


}

fn print_vector(x:Vec<i32>){
    println!("Inside print_vector function {:?}",x);
}
