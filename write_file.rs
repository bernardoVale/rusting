use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("output.txt").expect("Cannot create file");
    file.write_all(b"bello ma friend").expect("Can't write to the file");
}