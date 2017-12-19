use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    let content = read_hello().expect("Failed to read file content");
    println!("Content: {}", content);
}

fn read_hello() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut buffer = String::new();

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // this is almost the same thing as Golang if err := something; err != nil{ return err }
    };

    match f.read_to_string(&mut buffer) { // Return a result
        Ok(_) => Ok(buffer),
        Err(e) => Err(e)
    }
}

//Propagating errors with shortcut ;)

fn read_hello_shortcut() -> Result<String, io::Error> {
    let f = File::open("hello.txt")?; // Implicit match statement with `?`
    let mut buffer = String::new();
    f.read_to_string(&mut s)?; // Implicit match statement again
    Ok(s)
}

fn read_hello_improved() -> Result<String, io::Error> {
    let mut buffer = String::new();
    File::open("hello.txt")?.read_to_string(&mut buffer)?;
    Ok(buffer)
    //NOTE: the `?` operator can only be used in a function that returns `Result`
}