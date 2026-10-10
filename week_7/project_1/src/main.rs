// creating functions

use std::io;
fn get_input_user(prompt: &str) -> f64
{
    println!("{}",prompt );

    let mut input= String::new();
    io::stdin().read_line(&mut input).expect("Please input a valid number");
    input.trim().parse().expect("Please input valid input")   

}

fn cal_trapezium() -> f64{
    let h = get_input_user("Enter height: ");
    let b1 = get_input_user("Enter the first base: ");
    let b2 = get_input_user("Enter the second base: ");

    (h / 2.0) * (b1+b2)
}

fn cal_rhombus() -> f64{
    let d1 = get_input_user("Enter first diagonal: ");
    let d2 = get_input_user("Enter second diagonal: ");

    0.5 * d1 * d2
}

fn cal_parallelogram() -> f64{
    let b = get_input_user("Enter the base: ");
    let a = get_input_user("Enter the altitude: ");

    b * a
}

fn cal_cube() -> f64{
    let s = get_input_user("Enter the value for the side: ");

    6.0 * s * s
}

fn cal_cylinder() -> f64{
    let r = get_input_user("Enter the value for the radius");
    let h = get_input_user("Enter the value for the height");

    3.142857143 * r * r * h
}


// main menu and user selection
fn main() {
    println!("===== The Shape Calculator =====");
    println!("CODE |  SHAPE                |");
    println!("1    |  Trapezium            |");
    println!("2    |  Rhombus              |");
    println!("3    |  Parallelogram        |");
    println!("4    |  Cube (surface area)  |");
    println!("5    |  Cylinder (volume)    |\n");
    println!("PS: kindly note that the code for the first three shapes only solves for their area");
    println!("Dear user input your choice 1-5");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Please input a within 1-5.");
    let c:u32 = choice.trim().parse().expect("kindly input a value within 1-5.");


    match c {
        1 => {
            let result1 = cal_trapezium();
            println!("The are of the trapezium is: {}", result1);
        }
        2 => {
            let result2 = cal_rhombus();
            println!("The area of the rhombus is: {}", result2);
        }
        3 => {
            let result3 = cal_parallelogram();
            println!("The area of the Parallelogram is: {}", result3);
        }
        4 => {
            let result4 = cal_cube();
            println!("The surface area of the cube is: {}", result4);
        }
        5 => {
            let result5 = cal_cylinder();
            println!("The volume of the cylinder is: {}", result5);
        }
        _=>{
            println!("Invalid choice. Kindly choose a number 1 and 5");
        }
    }
}
