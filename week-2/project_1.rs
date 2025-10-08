//Written by Oluwatobiloba Zaccheaus
fn main(){
    let principal:f32=520000000.0;
    let rate:f32=10.0; //10% per annum
    let time:f32=5.0; //5 years
    let amount:f32=principal*(1.0+rate/100.0).powf(time);
    let compound_interest:f32=amount-principal;
    println!("The amount is {}, the compond interest is {}.",amount,compound_interest);
}