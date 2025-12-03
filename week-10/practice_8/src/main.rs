struct Employee{
    ceo:String,
    company:String,
    age:u32,
}
fn display(emp:Employee){
    println!("Name is : {}.\nCompany is {}.\nAge is {}.",emp.ceo,emp.company,emp.age);
}

fn main(){
    //Creeate a struct
    let emp1=Employee{company:String::from("Microsoft Corporation"),ceo:String::from("Satya Nadella"),age:56};

    let emp2=Employee{company:String::from("Goofle Inc."),ceo:String::from("Sundar Pitchai"),age:51};

    display(emp1);
    display(emp2);
}