use std::io::Write;      // This allows me to use the method of writing to my file


// Here I am writing the contents of my file and storing them to a variable 'drinks'
fn main(){ 
    let drinks = "LAGER, STOUT, NON-ALCOHOLIC
    33 Export, Legend, Maltina
    Desperados, Turbo King, Amstel Malta
    Goldberg, Williams, Malta Gold
    Gulder, ,Fayrouz
    Heineken, ,
    Stars, ,";

    // Here I am creating the file and pushing those contents into the file I created
    let mut file = std::fs::File::create("Nigerian Breweries.csv").expect("unable to create CSV file");
    file.write_all(drinks.as_bytes()).expect("Failed to write to file");

    // At the end if it is successful it prints the message below
    println!("Drinks Categories has been saved successfully!");


}
