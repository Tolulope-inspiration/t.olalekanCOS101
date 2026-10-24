use std::io::Write;
use std::fs::File;
use std::io::Read;

fn main() {

    let comm = vec![
        "Commissioners",
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etiye"
    ];

    let mut file1 = File::create("Commissioners.csv")
        .expect("Could not create file");

    for i in 0..comm.len() {
        let data = format!("{}\n", comm[i]);
        file1.write_all(data.as_bytes())
            .expect("Could not write to file");
    }

    println!("Commissioners file created successfully");


    let mini = vec![
        "Ministry",
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum"
    ];

    let mut file2 = File::create("Ministry.csv")
        .expect("Could not create file");

    for i in 0..mini.len() {
        let data = format!("{}\n", mini[i]);
        file2.write_all(data.as_bytes())
            .expect("Could not write to file");
    }

    println!("Ministry file created successfully");


    let zone = vec![
        "Zone",
        "South West",
        "North East",
        "South South",
        "South West",
        "South East"
    ];

    let mut file3 = File::create("Zone.csv")
        .expect("Could not create file");

    for i in 0..zone.len() {
        let data = format!("{}\n", zone[i]);
        file3.write_all(data.as_bytes())
            .expect("Could not write to file");
    }

    println!("Zone file created successfully");


    // Reopen the three files for reading
    let mut file1 = File::open("Commissioners.csv")
        .expect("Could not open Commissioners.csv");

    let mut file2 = File::open("Ministry.csv")
        .expect("Could not open Ministry.csv");

    let mut file3 = File::open("Zone.csv")
        .expect("Could not open Zone.csv");


    // Read the contents of the three files
    let mut commissioner = String::new();
    let mut ministry = String::new();
    let mut zone_data = String::new();

    file1.read_to_string(&mut commissioner)
        .expect("Could not read commissioners file");

    file2.read_to_string(&mut ministry)
        .expect("Could not read ministry file");

    file3.read_to_string(&mut zone_data)
        .expect("Could not read zone file");


    // Convert file contents into vectors
    let commissioners: Vec<&str> =
        commissioner.lines().collect();

    let ministries: Vec<&str> =
        ministry.lines().collect();

    let zones: Vec<&str> =
        zone_data.lines().collect();


    // Create the merged file
    let mut output = File::create("The EFCC-Merge.csv")
        .expect("Could not create merged file");


    // Write the headings
    writeln!(output, "Commissioner,Ministry,Zone")
        .expect("Could not write headings");


    // Merge the records using their indexes
    for i in 1..commissioners.len() {
        let merged_data = format!(
            "{},{},{}\n",
            commissioners[i],
            ministries[i],
            zones[i]
        );

        output.write_all(merged_data.as_bytes())
            .expect("Could not write merged data");
    }

    println!("The EFCC files have been merged successfully!");

}