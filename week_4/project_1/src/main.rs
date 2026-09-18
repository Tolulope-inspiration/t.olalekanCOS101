// Rust program to find roots of any quadratic equation

use std::io;

fn main() {
    // Friendly welcome message
    println!("Our very special user, welcome to the Quadratic Equation Solver! \nEnter your values for a, b, and c, and let's find the roots!");
    println!("Enter your value for 'a': ");


    // Inputing the value for a
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input for coefficient 'a'");
    let a:f64 = input1.trim().parse().expect("Failed to read input");


    // Ensuring the user gets a friendly error message instead of a bug
    if a==0.0{ 
        println!("'a' cannot be 0, please input another number", );return;
    }


    // Inputing the value for b
    println!("Enter your value for 'b': ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read the coefficient of 'b'");
    let b:f64 = input2.trim().parse().expect("Failed to read input");


    // Inputing the value of c
    println!("Enter your value for 'c'");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read the coefficient of 'c'");
    let c:f64 = input3.trim().parse().expect("Failed to read input");


    // Finding the discriminant
    let d:f64 = b*b - 4.0*a*c;
    println!("The discriminant of the quadratic equation is: {d}");


    // Conditions for the result of the discriminant
    if d > 0.0{
        let x1 = (-b + d.sqrt()) / (2.0*a);
        let x2 = (-b - d.sqrt()) / (2.0*a);
        println!("Your result will give you 'Two distinct roots.'");
        println!("Your first root is: {x1}");
        println!("Your second root is: {x2}");
       
    }
    // Another condition for the discriminant
    else if d == 0.0{
        let x = -b / (2.0*a);
        println!("Your result will give you 'Exactly one real root.'");
        println!("The root is {x}");
    }
    // Another condition for the discriminant
    else if d < 0.0{
        println!("No real roots");
    }

}
