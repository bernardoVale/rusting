fn main() {
   let number = 10;
   let num = Option::Some(10);

   match number {
      1 => println!("One"),
      2...20 => println!("Greater than one"),
      21 | 22 => println!("aloha"),
      _ => println!("Non")
   };

   match num {
       Some(2) => println!("It's two"),
       Some(_) => println!("It's something else"),
       None => println!("There's none")
   }
}