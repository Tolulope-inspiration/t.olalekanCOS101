// Declaring the struct 
#[allow(dead_code)] // I made a bit of research on how I can remove the warning 
                    //that is saying I did not make use of quantity and brand
                    //and including this removes that warning and prints a clean code.

struct Laptop{
    brand:String,
    quantity:u32,
    price_each:usize
}

// Here I  defining the logic to calculate the total cost for 3 piece from each brand
impl Laptop{
    fn cost_each(&self)->usize{
        self.price_each * 3
    }
}

fn main() {

    // Here I am initializing the structure
    let lap1 = Laptop{
        brand:String::from("HP"),
        quantity: 10,
        price_each: 650_000
    };

    let lap2 = Laptop{
        brand:String::from("IBM"),
        quantity: 6,
        price_each: 755_000
    };

    let lap3 = Laptop{
        brand:String::from("Toshiba"),
        quantity: 10,
        price_each: 550_000
    };

    let lap4 = Laptop{
        brand:String::from("Dell"),
        quantity: 4,
        price_each: 850_000
    };

    //Here I am printing the result to the customer: quantity and total cost
    println!("==================FOR HP LAPTOP==================");
    println!("Our very esteemed customer here is your receipt\nQuantity = 3\nTotal Cost = N{}\nThanks for your patronage👋\n",lap1.cost_each());

    println!("==================FOR IBM LAPTOP==================");
    println!("Our very esteemed customer here is your receipt\nQuantity = 3\nTotal Cost = N{}\nThanks for your patronage👋\n",lap2.cost_each());

    println!("==================FOR Toshiba LAPTOP==================");
    println!("Our very esteemed customer here is your receipt\nQuantity = 3\nTotal Cost = N{}\nThanks for your patronage👋\n",lap3.cost_each());

    println!("==================FOR Dell LAPTOP==================");
    println!("Our very esteemed customer here is your receipt\nQuantity = 3\nTotal Cost = N{}\nThanks for your patronage👋\n",lap4.cost_each());


}

