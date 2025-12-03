fn main() {
    let v=vec![20,40,60,80];
    //Vector v owns the object.

    let v2=v;
    let v2_return=display(v2);
    //println!("In main {:?}",v);
    //v is invalidated replace this with
    println!("In main {:?}",v2_return);
}


fn display(v:Vec<i32>)->Vec<i32>{
    //return same vector.
    println!("Inside display {:?}",v);
    return v;
}