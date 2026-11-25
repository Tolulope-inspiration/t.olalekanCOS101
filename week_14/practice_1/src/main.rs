use std::io::Read;
use std::fs::File;
use std::io;

// I created a funtion to display the entire database
fn admin(){
    let mut file = File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("Here is the entire database");
    print!("{}",contents);   
}


// I created a funtion to display the projects table
fn p_man(){
    let mut file = File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("Here is the projects table");
    print!("{}",contents);
}


// I created a funtion to display the staff table
fn emp(){
    let mut file = File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("Here is the staff table");
    print!("{}",contents);
}


// I created a funtion to display the customer table
fn cust(){
    let mut file = File::open("customer_table_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("*Here is the customer table*");
    print!("{}",contents);
}


// I created a funtion to display the data-plan table
fn ven(){
    let mut file = File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("Here is the data-plan table");
    print!("{}",contents);
}


// I am displaying all the category and their corresponding codes
fn main() {
    println!("\nDear user welcome to Globacom Nigeria. Kindly select your category\n");
    println!("CODE | CATEGORY         |");
    println!("-----+------------------+---");
    println!("1    | Administrator    |");
    println!("2    | Project Manager  |");
    println!("3    | Employee         |");
    println!("4    | Customer         |");
    println!("5    | Vendor           |");

    // I am asking for the user category
    println!("Enter your category: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Please enter within the range of 1 to 5");
    let c:i16 = input.trim().parse().expect("Enter between 1 and 5");


    // Here I am just making a condition based on what the user inputs in
    match c { 
        1 => admin(),
        2 => p_man(),
        3 => emp(),
        4 => cust(),
        5 => ven(),
        _ => println!("Invalid choice"), // This is to handle cases if user inputs outside the range
    }

}


