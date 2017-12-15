fn main() {
    let mut my_vector = vec![1, 2, 3, 4];
    //let my_vector: Vec<i32> = Vec::new();

    my_vector.push(49);
    my_vector.remove(1);

    for num in my_vector.iter() {
        println!("Num: {}", num);
    }
    println!("{}", my_vector[4]);
}