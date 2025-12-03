fn main() {
    //Initialize the vector.
    let mut user_purchases: Vec<Laptop> = vec![];
    //Add the User's purchases to the vector.
    Laptop::append_new(&mut user_purchases, "HP", 650_000, 3);
    Laptop::append_new(&mut user_purchases, "IBM", 755_000, 3);
    Laptop::append_new(&mut user_purchases, "TOSHIBA", 550_000, 3);
    Laptop::append_new(&mut user_purchases, "DELL", 850_000, 3);


    println!("This a program using Rust struct and method to calculate the total cost supposing a customer purchases 3 from each brand.\n");

    println!("User Orders:");
    //Now show the orders and totals
    {
        let mut total_price: u32 = 0;
        let mut total_units: u32 = 0;
        for i in user_purchases {
            println!("{}\n", i.format());
            total_price+=i.total();
            total_units+=i.units;
        }
        println!("-----------\nTOTALS\nTotal Units: {}\nTotal Price: ₦{}",total_units,total_price);
    }
}
struct Laptop {
    brand: &'static str,
    price: u32,
    units: u32,
}
impl Laptop {
    fn new(brand: &'static str, price: u32, units: u32) -> Self {
        Self {
            brand,
            price,
            units,
        }
    }
    fn append_new(v: &mut Vec<Laptop>, brand: &'static str, price: u32, units: u32) {
        v.push(Self::new(brand, price, units))
    }
    fn total(&self) -> u32 {
        self.price * self.units
    }
    fn format(&self) -> String {
        format!(
            "Brand: {}\nUnits: {}\nPrice: ₦{}\nCost: ₦{}",
            self.brand,
            self.units,
            self.price,
            self.total()
        )
    }
}
