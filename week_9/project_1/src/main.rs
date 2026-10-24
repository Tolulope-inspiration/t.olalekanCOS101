use std::fs::File;
use std::io::Write;

fn main() {
    println!("==============================");
    println!("WELCOME TO THE PAU-SIMS PORTAL");
    println!("==============================");

    let students: =[
        ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
        ("Adamu Aliyu", "ECO10110101", "Economics", 100),
        ("Shania Bolade", "CSC10328828", "Computer", 200),
        ("Adekunle Gold", "EEE11020202", "Electrical", 200),
        ("Bianca Edemoh", "MEE10202001", "Mechanical", 100),
    ];

    let mut file = File::create("Student_record.csv");
        .expect("Could not create file");
  
    writeln!(file, "Name,Matric Number,Department,Level")
        .expect("Failed to write header");

    for student in &students {
        writeln!(
            file,
            "{},{},{},{}",
            student.0,
            student.1,
            student.2,
            student.3
        )
        .expect("Failed to write student record");
    }

    println!("Student records have been saved successfully!");
}