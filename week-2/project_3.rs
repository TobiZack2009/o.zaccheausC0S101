//Written by Oluwatobiloba Zaccheaus
fn main(){
    let principal:f32=510000.0;
    let rate:f32=5.0;
    let time:f32=3.0;
    let amount:f32=principal*((1.0-(rate/100.0))).powf(time);
    let deprecation=principal-amount;
    println!("The deprecation is {}.",deprecation);
}