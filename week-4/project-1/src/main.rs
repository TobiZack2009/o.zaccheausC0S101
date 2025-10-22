use std::io;
fn main() {
    let mut a=String::new();
    let a:f32=io::stdin().read_line(&mut a).expect("s").trim().parse().unwrap() as f32;
}
