use std::fs;
use std::io;

fn read_file(filename: &str) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}
fn main() {
    let filename = "./crates/json_file_reader/input_json_file_reader.txt";

    match read_file(filename) {
        Ok(contents) => {
            println!("File contents:\n{}", contents);
        }
        Err(e) => {
            println!("Error reading file '{}': {}", filename, e);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_read_file_success() {
        let test_filename = "test_file.txt";
        let test_content = "This is a test file.";

        let mut file = fs::File::create(test_filename).unwrap();
        file.write_all(test_content.as_bytes()).unwrap();

        let result = super::read_file(test_filename);
        assert!(result.is_ok());
        assert!(result.unwrap() == test_content);

        // clean up the test file
        fs::remove_file(test_filename).unwrap();
    }

    #[test]
    fn test_read_file_not_found() {
        let non_exsistent_filename = "non_existent_file.txt";

        let result = super::read_file(non_exsistent_filename);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::NotFound);
    }
}
