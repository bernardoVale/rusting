fn main() {
   let s1 = String::from("hello");
   let s2 = s1;

   /* Generates compile error because the value of s1 has moved to s2
      Rust does that to prevent us from releasing memory twice that could lead 
      to memory corruption
   */ 
   println!("{}", s1);
}