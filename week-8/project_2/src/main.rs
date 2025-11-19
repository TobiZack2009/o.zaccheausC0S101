use std::io;

fn main() {
    let mut submissions:Vec<(String,String,usize)>=vec![];

    println!("Hello! This is a program by Ernst & Young (EY) Global Limited to find the person with the highest years of programming experience during the job interview. Enter eq to find the most experienced developer.");
    loop{
        submissions.push((input_str("Candidate Name:"),input_str("Candidate skills: "),input_number("Number of years of experience of Candidate (a number)")));
        if submissions[submissions.len()-1].0=="eq".to_string() || submissions[submissions.len()-1].1=="eq".to_string() 
        {
            submissions.remove(submissions.len()-1); //Remove the last item on the submissions vector
            let mut max_item=0usize;
            let mut max_index=0usize;
            for i in 0..submissions.len() {
                let (name,skills,years)=&submissions[i];
                if (*years<max_item) {
                    max_item=*years;
                    max_index=i as usize;
                }
            }
            println!("The candidate with the highest experience is {}. His skills are {}. They have {} years of experience.",submissions[max_index].0,submissions[max_index].1,submissions[max_index].2);
        }

    }
}

fn input_str(req:&str)-> String{
    println!("{req}");
    let mut input=String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    return input.trim().to_string();


}
fn input_number(req:&str)->usize{

    input_str(req).parse().unwrap_or_else(|x| input_number(req))
    
}