fn main() {
    let mut friends=vec!["Mary","Sam","Sally","Greg","Ade","Mark","June","Ife"];

    let mut ages=vec![16,17,19,22,20,21,18,23];

    println!("\nAge allocation\n");

    for i in 0..ages.len() 
    {
        println!("{} is {} years old\n",friends[i],ages[i]);
    }
}
