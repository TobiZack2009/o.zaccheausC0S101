use std::io;
fn main() {
    println!("Hello welcome to the PAU orering system.\nEnter the code for the item you want to order, then the quantity of the item you want to order, for example like this: \nP4-This orders 4 Poundo Yam/Edinkaiko Soup. The maximum number of items that can be ordered at once is 9.");
    println!("┌──────┬────────────────────────────┬───────────┐\n│ Code │            Menu            │   Price   │\n├──────┼────────────────────────────┼───────────┤\n│ P    │  Poundo Yam/Edinkaiko Soup │    N3,200 │\n│ F    │  Fried Rice & Chicken      │    N3,000 │\n│ A    │  Amala & Ewedu Soup        │    N2,500 │\n│ E    │  Eba & Egusi Soup          │    N2,000 │\n│ W    │  White Rice & Stew         │    N2,500 │\n└──────┴────────────────────────────┴───────────┘\nTo stop ordering and see your final price, simply press Enter!");
    let mut total_price=0.0;

   'main:  loop{

        //Create a string for user input.
        let mut user_input=String::new();
        io::stdin().read_line(&mut user_input).expect("Error reading line!");
        user_input=String::from(user_input.to_uppercase().trim());


        //It nothing is entered, break the loop.
        if user_input.len()==0 {
            break 'main;
        }
        //Else, give an error if the length of the string is not equal than 2.
       else if user_input.len()!=2 {
            println!("Enter a code and a quantity in the valid order!");
            user_input.clear();
            continue 'main;}
       
       //Get the first two charcters containing the order code and number of items ordered,
        let mut user_input_chars=user_input.chars();
        let order_code=user_input_chars.next().unwrap();
        let order_count=user_input_chars.next().unwrap().to_digit(10).unwrap() as f64;
        
       //Based on the code, add a different price to the total price. Also if a character that is not a menu code is the first character, prompt the user to make an input.
       total_price+= match order_code {
        'P'=>3200.0,
        'F'=>3000.0,
        'A'=>2500.0,
        'E'=>2000.0,
        'W'=>2500.0,
        _=>{
            println!("Enter a valid character for the menu code.");
            continue 'main
        }
        }*order_count;
    }

// If the total order is greater than N10,000, give a  discount of 5%.

if total_price>10000.0 {
    total_price*=0.95;
}

println!("Total price is ₦{}.\nThank you for time!\nhave a nice day!",total_price)
    
}
