use std::fs::File;
use std::io::Read;

fn read_file(file_path: &str) {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            return;
        }
    };
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        println!("Error reaidng file: {}", e);
        return;
    }
    println!("File contents:\n{}", contents);
}

fn main() {
    const FILE_PATH: &str = "assets/data.txt";
    read_file(FILE_PATH);
}
