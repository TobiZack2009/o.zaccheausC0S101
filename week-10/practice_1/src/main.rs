fn main() {
    let v=vec![101,250,330,400];
//v2 borrows instead of owning.
    let v2=&v;


    println!("{:?}",v);
}
