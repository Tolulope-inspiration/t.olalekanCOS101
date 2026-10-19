use std::io::Write;
use std::fs;

fn main(){

    let announce = "Week 9 - Rust File Input & Output\n";
    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all(announce.as_bytes()).expect("write failed");
    fs::remove_file("data.txt").expect("counld not remove file");
    println!("file is removed");
    println!("Data written to file");
}
