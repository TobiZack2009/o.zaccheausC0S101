use std::{collections::*, *,io::*};
fn main() {
    clear_screen();
    let dbases: HashMap<&str, &str> = [("db", include_str!("databases/globacom_db.sql")),
    ("customer",include_str!("databases/globacom_project_customer.sql")),
    ("dataplan",include_str!("databases/globacom_project_dataplan.sql")),
    ("staff",include_str!("databases/globacom_project_staff.sql")),
    ("table",include_str!("databases/globacom_project_table.sql"))].into();
    

    typewrite(200, "Welcome to the program. Enter your username and password.\nUsername: ".into());
    let username=input();
    typewrite(200, "Password: ".into());
    let password=input();
    println!("{}{}",&username,&password);
    


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