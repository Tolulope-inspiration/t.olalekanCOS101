use std::io;

fn main() {

    println!("Enter a number");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let mut num:i32 = input1.trim().parse().expect("Failed to input");

    while num < 10 {

        println!("Inside loop number value is {num}");
        num += 1;
    } 
    println!("outside loop number value is {num}");


    println!("Enter a number");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let mut age:i32 = input2.trim().parse().expect("Failed to input");

    while age < 90{
        println!("Inside loop number value is {age}");
        age +=1;
    }

}
