use std::io;

fn main() {

    println!("Dear user welcome to the \n\n======== Federal Government of Nigeria APS level checker ========");


    // Here I am collecting the years of experience of the user
    println!("Enter your years of experience: ");
    let mut years = String::new();
    io::stdin().read_line(&mut years).expect("Please input a valid years of experience");
    let y:u16 = years.trim().parse().expect("Kindly input a valid years of experience");


    // I declared the vector to contain all the APS levels
    let p = vec!["APS 1-2", "APS 3-5", "APS 6-8", "EL1 9-10", "EL2 10-13", "SES"];


    // conditions to check the user's experience
    if y >=1 && y <=2{
        println!("Dear user your APS level is: {}",p[0]);
        println!("Thanks for using the APS level checker 👋");
    }
    else if y >=3 && y <=5{
        println!("Dear user your APS level is: {}",p[1]);
        println!("Thanks for using the APS level checker 👋");
    }
    else if y >=6 && y <=8{
        println!("Dear user your APS level is: {}",p[2]);
        println!("Thanks for using the APS level checker 👋");
    }
    else if y >=9 && y <=10{
        println!("Dear user your APS level is: {}",p[3]);
        println!("Thanks for using the APS level checker 👋");
    }
    else if y >=11 && y <=13{
        println!("Dear user your APS level is: {}",p[4]);
        println!("Thanks for using the APS level checker 👋");
    }
    else if y >=14 && y <=85{
        println!("Dear user your APS level is: {}",p[5]);
        println!("Thanks for using the APS level checker 👋");
    }
    else {
        println!("Please input a valid years of experience");
    }


}
