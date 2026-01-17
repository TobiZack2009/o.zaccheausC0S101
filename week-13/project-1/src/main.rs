use std::{collections::*, io::*, *};
fn main() {
    let dbases: HashMap<&str, &str> = 
    [("Database Structure", include_str!("databases/globacom_db.sql")),
    ("customer",include_str!("databases/globacom_project_customer.sql")),
    ("dataplan",include_str!("databases/globacom_project_dataplan.sql")),
    ("staff",include_str!("databases/globacom_project_staff.sql")),
    ("table",include_str!("databases/globacom_project_table.sql"))].into();
    

    loop{
    clear_screen();
    typewrite(100, "Welcome to the program.\nEnter your username and password.\n\nUsername: ".into());
    let username=input();
    typewrite(100, "Password: ".into());
    let password=input();
    let [key,name]=
    match format!("{}{}",username,password).as_str() {
        "Charlie Brownwwee"=>["Database Structure","Admin"],
        "John Joopwd"=>["customer","Customer"],
        "Jul SusieJoke"=>["dataplan","Vendor"],
        "John JacobNoo"=>["staff","Employee"],
        "Tim TomNie"=>["table","Project Manager"],
        _=>{
            typewrite(100, "Enter a valid username and password".into());
            continue;
        }
    };

    {
        let filename=format!("{}.sql",key);
        match  fs::write(filename.clone(), dbases.get(key).unwrap()){
            Ok(val)=>{
                println!("\nThe database will be saved as {}.",filename.clone());
            }
            Err(_)=>{
                eprintln!("Failed to save the file {}.",filename)
            }
        };
        typewrite(100, format!("\nYou are a {}.\nHere is the database data.",name));
        wait(1000);
        typewrite(100, format!("{}",dbases.get(key).unwrap()));
        break;

    }


    

    }
}

fn clear_screen(){
    print!("\x1B[2J\x1B[H");
}

fn wait(time:u32) {
    thread::sleep(time::Duration::from_millis(time.into()));
}
fn flush(){
    stdout().flush();
}

fn typewrite(duration:u32,text:String){
    let chars=text.chars();
    for i in chars {
        flush();
        print!("{}",i);
        wait(duration);
        flush();
    }
}

fn input()->String{
    let mut x:String=String::new();
    io::stdin().read_line(&mut x);
    x.trim().into()
}