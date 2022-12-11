use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_file_to_string(file_name: String) -> String {
    let path = Path::new(&file_name);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut string = String::new();

    match file.read_to_string(&mut string) {
        Err(why) => panic!("Couldn't read {}:{}", display, why),
        Ok(_) => string,
    }
}
