fn main() {
    let mut s2 = String::from("foo");
    borrow_variable(&s2); // Sending a reference -- this is called borrowing in rust

    println!("s2 {}", s2);

    // Worths mentioning that you can have only one mutable reference to a particular piece of data
    // this allows Rust prevent data races at compile time.
    borrow_mutable_variable(&mut s2);
    println!("s2 {}", s2);
}


fn borrow_variable(s: &String) {
    println!("Borrow {}", s);
}

// If we need to change the value we need to create a "mutable reference"
fn borrow_mutable_variable(s: &mut String){
    s.push_str("bar");
}