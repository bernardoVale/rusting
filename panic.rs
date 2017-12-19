fn main() {
    // Panic is an unrecorable error. Call this macro to exit immetiatly!
    // panic!("Run to the hills"); // explicit call

    let v = vec![1, 2, 3];
    v[100]; // Rust will call panic implicitly since this exceptioncan't be prevented at compile time
}