use std::fs::File;
use std::io::Read;

pub fn get_file_separated_or_panic(file_name: &str, sep: &str) -> Vec<String> {
    let mut file = File::open(file_name).expect("Invalid file");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("File could not be read.");
    file_contents.split(sep).map(|e| e.to_string()).collect()
}
