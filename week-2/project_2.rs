//Written by Oluwatobiloba Zaccheaus
fn main(){
    let sales=[450000.0,1500000.0,750000.0,2850000.0,250000.0];
    let mut sum=0.0;
    for item in sales{
        sum+=item;
    };
    println!("The sum is {}.",sum);
    println!("The average is {}.",sum/(sales.len() as f32))
}