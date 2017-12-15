fn main() {
   let s1 = String::from("hello"); // s1 comes into scope
   
   takes_ownership(s1); // s1 is now out of scope

   println!("{}", s1); // compile error - value has moved
}

fn takes_ownership(arg: String) {
    println!("{}", arg);
}