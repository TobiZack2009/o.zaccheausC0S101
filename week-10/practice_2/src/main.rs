fn main() {
    let v=vec![10,20,30];
    //V owns the object.

    let v2=v; //Changed move to borrow. 

    display(v2.to_vec());
    //v2 is moved to display and invalidated. This is chsnged to a copy.

    println!("In main {:?}",v2);
    //v2 is unusable here.
}

fn display(v:Vec<i32>){
    println!("Inside display {:?}",v)
}
