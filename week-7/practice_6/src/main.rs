fn main() {
   let mut num=5i32;
   mutate_num_to_zero(&mut num);
   println!("The value of no is: {}",num);
}

fn mutate_num_to_zero(param_num:&mut i32){
    *param_num=*param_num*0;
    println!("param_num variable is: {param_num}");
}