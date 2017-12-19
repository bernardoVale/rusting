use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap(); // Return the ok variant if ok otherwise call panic!
    let f = File::open("hello.txt").expect("Failed to open a file"); // Same thing but you can specify an error message
}