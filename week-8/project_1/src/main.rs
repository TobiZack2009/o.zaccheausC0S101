use std::io;
fn main() {
    let office_admin_range=vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];
    let profession=vec!["Office Administrator","Academic", "Lawyer" ,"Teacher"];
    let academic_range=vec!["-","Research Assistant","PhD Candidate","Post-Doc Researcher","Senior Lecturer ","Dean"];

    let lawyer_range=vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner"];

    let teacher_range=vec!["Placement","Classroom Teacher","Snr Teacher","Leading Teacher","Deputy Principal","Principal"];

    fn exprience_to_index(years:i64)->usize{
      match years {
        i64::MIN..1=>0,
        1..=2=>0,
        3..5=>1,
        5..8=>2,
        8..10=>3,
        10..=13=>4,
        13..=i64::MAX=>5,
        }
    }

    println!("Hello! This is a program to validate Staff Level!\nWhat is your profession?\nPick one of the following:\n1) {}\n2) {}\n3) {}\n4) {}",profession[0],profession[1],profession[2],profession[3]);
    let mut staff_type=String::new();
    loop{
      staff_type.clear();
      io::stdin().read_line(&mut staff_type);
      if profession.contains(&staff_type.trim()){
        break;
      }
      continue;
      }
      staff_type=staff_type.trim().to_string();

      let experience_years:i64;
      println!("Enter the number of your experience.");
      loop{
        let mut age_str=String::new();
        io::stdin().read_line(&mut age_str).expect("Failed to read line!");
        match age_str.trim().parse() {
          Ok(val)=>{
            experience_years=val;
            break;
          },
          Err(_)=>{continue;},
        }
      }
      let index=exprience_to_index(experience_years);

      println!("Your position is: {}",
      match  staff_type.as_str() {
        "Office Administrator"=>office_admin_range[index],
        "Acadmic"=>academic_range[index],
        "Lawyer"=>lawyer_range[index],
        "Teacher"=>teacher_range[index],
        _=>todo!()
  
      }
    
    
    )

      

}
