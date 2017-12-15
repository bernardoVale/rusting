fn main() {
    let mut my_string = String::from("Hey ho!");

    println!("Lenght: {}", my_string.len());
    println!("Is empty? {}", my_string.is_empty());

    for token in my_string.split_whitespace() {
        println!("{}", token);
    }

    println!("Contains hey? {}", my_string.contains("Hey"));

    my_string.push_str(" more something"); // mutating a string

    println!("{}", my_string);
}