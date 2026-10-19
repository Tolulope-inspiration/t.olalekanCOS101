use std::fs::OpenOptions;
use std::io::Write;


fn main() {

    let mut file = std::fs::File::create("file_4.txt").expect("create failed");
    file.write_all("Welcome to Rust programming language".as_bytes()).expect("write failed");

    let mut file = OpenOptions::new().append(true).open("file_4.txt").expect("Cannot open file");
    file.write_all("\nHello Class".as_bytes()).expect("write failed");
    file.write_all("\nThis is the appendage to the document."
        .as_bytes()).expect("write failed");
    println!("file append success");

}
