use std::fs;
fn main() {
    fs::remove_file("../files/data.txt").expect("Failed to delete");
    println!("THe file was deleted!");
}
