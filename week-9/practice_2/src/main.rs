use std::io::Read;
fn main() {
    let mut file=std::fs::File::open("../files/welcome_message.txt").expect("Failed to write!");
    let mut contents=String::new();
    file.read_to_string(&mut contents).expect("Failed to read file!");
    println!("{}",contents);


}
