use std::io::Write;
fn main() {
    let annouce="\nWeek 9 - Rust File Input and Output";
    let dep="Department of Computer Science\n";

    let mut file=std::fs::File::create("../files/data.txt").expect("Creation failed!");
    file.write_all("Welcome to Rust Programming\n".as_bytes()).expect("Creation Failed");
    file.write_all(annouce.as_bytes()).expect("Write Failed!");
    file.write_all(dep.as_bytes()).expect("Write failed!");
    println!("Data Written to file!");


}
