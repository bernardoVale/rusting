
fn main (){
    print_numbers_to(20);
    if is_even(30) {
        println!("Is even");
    }
}

fn is_even(num: u32) -> bool {
    return num % 2 == 0;
}

fn print_numbers_to(limit: u32) {
    for n in 1..limit {
        if is_even(n) {
            println!("Is even!");
            continue;
        }
        println!("It is odd");
    }
}