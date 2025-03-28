use std::{fs::OpenOptions, io::Write};

fn main() {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("dummy_text.txt");

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
