fn main() {
   let s1 = String::from("hello");
   let s2 = s1.clone();
   println!("{}", s1);

   // Because integers are stack-only data we don't need this operation
   let int1 = 2;
   let int2 = int1;

   println!("{}", int1);
}