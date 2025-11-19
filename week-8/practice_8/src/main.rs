fn main() {
    let mut mountain_heights=("Everest",8848,"Fishtail",6993);

    println!("Optional tuple = {:?}",mountain_heights);

    mountain_heights.2="Lhotse";
    mountain_heights.3=8516;

    println!("Changed Tuple = {:?}",mountain_heights);
}
