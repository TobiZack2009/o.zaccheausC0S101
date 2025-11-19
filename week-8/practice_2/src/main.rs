fn main() {
    let comp=vec!['C','O','M','P','U','T','E','R'];

    let mut input1=String::new();

    println!("Enter an index value between (0-7)");
    std::io::stdin().read_line(&mut input1).expect("Failed to read line!");

    let index:usize=input1.trim().parse().expect("Failed to read input!");

    let ch:char=comp[index];
    

    println!("{} is character for index [{}]\n",ch,index);
}
