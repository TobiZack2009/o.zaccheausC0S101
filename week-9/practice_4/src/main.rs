use std::fs::OpenOptions;
use std::io::Write;

fn main(){
    let mut file=OpenOptions::new().append(true).open("../files/data.txt").expect("Cannot open file");
    file.write_all("\nHello Class".as_bytes()).expect("Write failed");
    file.write_all("\nThis appendage to the document".as_bytes()).expect("Failed to write.");
    println!("FIle append success.");
}