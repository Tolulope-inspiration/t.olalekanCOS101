// These are the libraries I made use of in my program

use chrono::Local;              //This is to get the time and date from the users device
use chrono::Timelike;           //This is to extract the time this case I just extracted the hour
use std::io;                    // This is to collect input from the user

// I created this function to greet the user depending on the current hour they use the program.
fn greet_user() {
    let hour = Local::now().hour();

    if hour < 12 {
        println!("\nGood morning! Dear applicant");
    } else if hour < 17 {
        println!("\nGood afternoon! Dear applicant");
    } else {
        println!("\nGood evening! Dear applicant");
    }
}



fn main() {
    // Here I am calling the funtion to greet the user
    greet_user();

    // I am storing the user's inputs into a vector that can take both their name and years of experience
    let mut applicants: Vec<(String, String, u32)> = Vec::new();

    loop{
        
        // Here I am collecting the user's surname
        println!("Enter your Lastname(surname): ");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Kindly input a valid name");


        // Here I am collecting the user's firstname
        println!("\nEnter your Firstname: ");
        let mut name1 = String::new();
        io::stdin().read_line(&mut name1).expect("Kindly input a valid name");


        // Here I am collecting the user's years of experience
        println!("\nEnter your years of experience");
        let mut years = String::new();
        io::stdin().read_line(&mut years).expect("Kindly input a valid years of experience");
        let y:u32 = years.trim().parse().expect("Invalid years of experience");
        

        //I included this condition so that the user don't include years below 1 or above 85
        if y < 1 || y > 85{
            println!("Dear applicant, experience must be between 1 and 85");
        }
        else{
           applicants.push((name.trim().to_string(), name1.trim().to_string(), y));
        }

        println!("Do you want to add another applicant? (yes/no)");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read input");
        if answer.trim().eq_ignore_ascii_case ("no"){
            break;
        }
        else{
            println!("input yes or no");
            break;
        }
        
    }

    //I used the for loop to go through each of the applicants and display their information
    for applicants in &applicants{
        println!("\nName: {} {}\nExperience: {}years",applicants.0.trim(),applicants.1.trim(),applicants.2);
        println!("Thank you for applying 👋\n");
    }

    // This condition here is to display the person with highest years of experience
    if !applicants.is_empty() {
        let mut highest = &applicants[0];

        for applicants in &applicants {
            if applicants.2 > highest.2 {
                highest = applicants;
            }
        }
        
        println!("\nApplicant with the highest experience:");
        println!("Name: {} {}", highest.1, highest.0);
        println!("Experience: {} years", highest.2);
    }
    else{
        println!("No valid applicants were added");
    }

}
