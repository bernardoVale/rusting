fn main() {
    let animals = vec!["Rabbit", "Dog", "Cat"];

    for (index, a) in animals.iter().enumerate() {
        println!("Animal #{} name: {}", index, a);
    }
}
