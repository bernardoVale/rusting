use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(e) => {
            panic!("Failed to open a file: {:?}", e);
        }
    };
}