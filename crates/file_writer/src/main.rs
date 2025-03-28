use std::{env, fs::OpenOptions, io::Write};

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Error: No file path provided");
        std::process::exit(1);
    });

    let file = OpenOptions::new().append(true).create(true).open(&path);

    let data = "Test Line\n";
    match file {
        Ok(mut file) => match file.write_all(data.as_bytes()) {
            Ok(_) => (),
            Err(error) => match error.kind() {
                std::io::ErrorKind::Interrupted => {
                    println!("interrupted while writing");
                }
                _ => {
                    println!("error while writing: {}", error);
                }
            },
        },
        Err(error) => match error.kind() {
            std::io::ErrorKind::InvalidInput => {
                println!("wrong input options in the code");
            }
            _ => {
                println!("error while opening the file: {}", error);
            }
        },
    };
}
