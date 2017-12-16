fn main() {
    let dangling_var = dangle();
}

fn dangle() -> &String {
    let s = String::from("Foo");
    /*
    I'm returning a pointer to s, but since
    s will go out of scope, Rust will free it's memory
    therefore it would be pointing to a piece of memory
    that no longer exists, this is called dangling
    reference.

    Dangling reference does not exists in rust, this code
    won't compile.
    */
    &s 
}