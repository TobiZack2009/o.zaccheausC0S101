fn main() {
    let numbers=[1,2,3,4,5];
    println!("Original array = {:?}",numbers);
    
    let slice1=&numbers[1..3];
    println!("2nd and 3rd elements sliced={:?}",slice1);

    let slice2=&numbers[..3];
    println!("0 to 3 index sliced={:?}",slice2);

    let slice3=&numbers[2..];
    println!("2 to 5 index sliced={:?}",slice3);

    let slice4=&numbers[..];
    println!("Whole array sliced!");
}
