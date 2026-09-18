// The Incentive Calculator

use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Hello and welcome our dear employee.");
    println!("Type in yes or no");
    println!("Are you experienced? ");

    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");
    let a:&str = input1.trim();

    if a == "yes"{
        println!("You are experienced, that is very good.");
    }
    else if a == "no"{
        println!("You are not experienced");
        println!("This is the annual incentive for your experience: N100_000.0");
    }
    
    println!("Dear employee enter your age: ");

    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input");
    let b:f32 = input2.trim().parse().expect("Please type in a valid age");

    if b >= 40.0{
        println!("This is the annual incentive for your experience: N1_560_000.0");
    }
    else if b >= 29.0 && b <=39.0{
        println!("This is the annual incentive for your experience: N1_480_000.0");
    }
    else if b >= 1.0 && b <= 28.0{
        println!("This is the annual incentive for your experience: N1_300_000.0");
    }
    else if b <= 0.0{
        println!("Please enter a valid age.");
    }
    
}
