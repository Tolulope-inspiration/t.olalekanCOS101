// Rust program - The Incentive Calculator

use std::io;

fn main() {
    // Collecting input from user
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Hello and welcome our dear employee.");
    println!("Type in yes or no");
    println!("Are you experienced? ");

    // Read the user's response, remove the extra spaces, and store the cleaned input in the variable 'a'.
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");
    let a:&str = input1.trim();


    // Conditions for the input of 'a'
    if a == "yes"{
        println!("You are experienced, that is very good.");
    }
    else if a == "no"{
        println!("You are not experienced");
        println!("This is the annual incentive for your experience: N100,000");
        return;
    }
    else {
        println!("Dear user type either yes or no");
        return;
    }
    
    
    // Second question to ask the user
    println!("Dear employee enter your age: ");

    // Read the user's response, remove the extra spaces, and store the cleaned input in the variable 'b'.
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input");
    let b:f32 = input2.trim().parse().expect("Please type in a valid age");

    // Conditions statement for 'b' and the corresponding output
    if b >= 40.0{
        println!("This is the annual incentive for your experience: N1,560,000");
    }
    else if b >= 29.0 && b <=39.0{
        println!("This is the annual incentive for your experience: N1,480,000");
    }
    else if b >= 1.0 && b <= 28.0{
        println!("This is the annual incentive for your experience: N1,300,000");
    }
    else if b <= 0.0{
        println!("Please enter a valid age.");
    }
    
}
