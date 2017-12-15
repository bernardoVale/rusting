fn main() {
    let tup1 = (20, "Rust", 30, 35, false, (20, true, "hey")); // tuple can store multiple data types
    let tup2 = ("foo", "bar", "biz");
    let (a, b, c) = tup2; // 

    println!("a is {} b is {} c is {}", a, b, c);

    println!("{}", tup1.4); // accessing tuple index
    println!("{}", (tup1.5).1); // access tuple index inside tuple


}