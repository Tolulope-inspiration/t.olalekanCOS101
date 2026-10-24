use std::io::Write;
use std::fs::File;

fn main() {
    println!("==============================");
    println!("WELCOME TO THE PAU-SIMS PORTAL");
    println!("==============================\n");
    println!("NAME          | MATRIC NUMBER | DEPARTMENT | LEVEL");
    println!("Oluchi Mordi  | ACC10211111   | Accounting | 300");
    println!("Adamu Aliyu   | ECO10110101   | Economics  | 100");
    println!("Shania Bolade | CSC10328828   | Computer   | 200");
    println!("Adekunle Gold | EEE11020202   | Electrical | 200");
    println!("Bianca Edemoh | MEE10202001   | Mechanical | 100");

    let students: Vec<(&str,&str,&str,u16)> = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
        ("Adamu Aliyu", "ECO10110101", "Economics", 100),
        ("Shania Bolade", "CSC10328828", "Computer", 200),
        ("Adekunle Gold", "EEE11020202", "Electrical", 200),
        ("Bianca Edemoh", "MEE10202001", "Mechanical", 100)
    ];

    let headers = ["Names", "Matriculation Number", "Department", "Level"];

    let mut file =File::create("Student_record.csv").expect("could not file");
    file.write_all(format!("{}, {}, {}, {}\n", headers[0], headers[1], headers[2], headers[3]).as_bytes()).unwrap();
    
    for a in 0..=4{
    file.write_all(format!("{}, {}, {}, {}\n", students[a].0, students[a].1, students[a].2, students[a].3).as_bytes     ()).expect("Failed to write file");    
    }
    println!("\nStudent record has been saved successfully!");
}