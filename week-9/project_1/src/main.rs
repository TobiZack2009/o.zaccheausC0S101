use std::fs::*;
use std::io::Write;
fn main() {
    let leger_bottles=["33 Export","Desperados","Goldberg","Heineken","Star"];
    let stout_bottles=["Legend","Turbo King","Williams"," "," "];
    let non_alcoholic_bottles=["Maltina","Amstel Malta","Malta Gold","Fayrouz"," "];
    let mut saved_string="Leger,Stout,Non-Alcoholic".to_string();


    for i in 0..5 {
        saved_string.push_str(format!("\n{},{},{}",leger_bottles[i],stout_bottles[i],non_alcoholic_bottles[i]).as_str());
}
    File::create("../files/bottles.csv").expect("Failed to create file!").write_all(saved_string.as_bytes()).expect("Failed to write!");

    println!("Successfully created bottles.csv!")


}
