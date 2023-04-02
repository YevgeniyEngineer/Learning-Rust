use std::fs::File;
use std::io::Error;
use std::io::Read;

fn main() {
    match read_file_contents("file.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {:?}", e),
    }
}

fn read_file_contents(file_path: &str) -> Result<String, Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
