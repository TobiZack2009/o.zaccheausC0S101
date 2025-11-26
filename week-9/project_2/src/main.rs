use std::io::Write;





fn main() {
    let student_data=[("Oluchi Mordi","ACC10211111","Accounting",300),("Adams Aliyu","ECO10110101","Economics",100),("Shania Bolade","EEE11020202","Computer",200),("Adekunle Gold","EEE11020202","Electrical",200),("Blanca Edemoh","MEE10202001","Mechanical",100)];
    let mut student_data_csv="Student Name, Matric Number, Department, Level\n".to_string();

    for i in student_data {
        student_data_csv.push_str(format!("{},{},{},{}\n",i.0,i.1,i.2,i.3).as_str());

    }
    std::fs::File::create("../files/students.csv").expect("Failed to create a file!").write_all(student_data_csv.as_bytes()).expect("Failed to create a file!");
}
