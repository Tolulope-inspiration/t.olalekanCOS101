// Rust program for a Restaurant Menu - PAU cafe

use std::io;
fn main() {
    println!("Hello and welcome our very special customer to PAU cafeteria app, here are our menu:");
    println!("There would be a 5% discount if you purchase 10,000 and above\n");                                      
    println!("============= PAU Cafe Menu ==============");
    println!("CODE | FOOD ITEM                   | PRICE");
    println!("P    | Poundo Yam / Edinkaiko Soup | N3,200");
    println!("F    | Fried Rice & Chicken        | N3,000");
    println!("A    | Amala & Ewedu               | N2,500");
    println!("E    | Eba & Egusi                 | N2,000");
    println!("W    | White Rice & Stew           | N2,500");

    // Collecting the code and quantity from user
    println!("Input your code: ");
    let mut code = String::new();
    io::stdin().read_line(&mut code).expect("Incorrect code");
    let c:char = code.trim().to_uppercase().parse().expect("Input a valid code");

    println!("Input your quantity: ");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Incorrect value");
    let q:f64 = quantity.trim().parse().expect("Input a valid quantity");

    // Calculating the actual price without discount
    let p1 = q * 3200.0;
    let f2 = q * 3000.0;
    let a3 = q * 2500.0;
    let e4 = q * 2000.0;
    let w5 = q * 2500.0;

    if c == 'P'{
        println!("Total amount is: {}", p1);

        if p1 > 10_000.0{ //Condition if the user purchase more than N10,000
            let d1 = p1 - p1 * 0.05;
            println!("Thank you our dear customer for purchasing up to 10,000");
            println!("We have added a discount of 5%\nYour new price is N{}", d1); 
        }
    }

    if c == 'F'{
        println!("Total amount is: {}", f2);

        if f2 > 10_000.0{ //Condition if the user purchase more that N10,000
            let d1 = f2 - f2 * 0.05;
            println!("Thank you our dear customer for purchasing up to 10,000");
            println!("We have added a discount of 5%\nYour new price is N{}", d1); 
        }
    }

    if c == 'A'{
        println!("Total amount is: {}", a3);

        if a3 > 10_000.0{ //Condition if the user purchase more that N10,000
            let d1 = a3 - a3 * 0.05;
            println!("Thank you our dear customer for purchasing up to 10,000");
            println!("We have added a discount of 5%\nYour new price is N{}", d1); 
        }
    }

    if c == 'E'{
        println!("Total amount is: {}", e4);

        if p1 > 10_000.0{ //Condition if the user purchase more that N10,000
            let d1 = e4 - e4 * 0.05;
            println!("Thank you our dear customer for purchasing up to 10,000");
            println!("We have added a discount of 5%\nYour new price is N{}", d1); 
        }
    }

    if c == 'W'{
        println!("Total amount is: {}", w5);

        if p1 > 10_000.0{ //Condition if the user purchase more that N10,000
            let d1 = w5 - w5 * 0.05;
            println!("Thank you our dear customer for purchasing up to 10,000");
            println!("We have added a discount of 5%\nYour new price is N{}", d1); 
        }
    }
}
