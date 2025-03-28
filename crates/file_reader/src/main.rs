use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("doesnt_exist.txt");
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                println!("{}", line.unwrap())
            }
        }
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                println!("❌ File not found: {}", error)
            }
            _ => {
                println!("Error in opening file: {}", error)
            }
        },
    };
}
