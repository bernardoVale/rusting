struct Person {
  name: String,
  age: u8
}

trait ToMothafucker {
    fn to_mothafucka(&self) -> String;
}

impl ToMothafucker for Person {
    fn to_mothafucka(&self) -> String {
      return format!("My Name is mothafucka {} and I am {}", self.name, self.age);  
    }
}

impl ToString for Person {
  fn to_string(&self) -> String {
    return format!("My Name is {} and I am {}", self.name, self.age);
  }
}

fn main() {
    let berna = Person { name: String::from("Bernardo Vale"), age: 25 };

    println!("{}", berna.to_string());
    println!("{}", berna.to_mothafucka());
}