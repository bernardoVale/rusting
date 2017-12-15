fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let numbers2: [i32; 5] = [1,2,3,4,5] // with data type specification
    let numbers3 = [2; 400]; // 400 items with value 2

    for n in numbers.iter() {
        println!("N: {}", n);
    }

    for n in 0..numbers.len() {
        println!("N: {}", numbers[n]);
    }
}