fn main() {
    let mut x = 10;
    /* you can't define an immutable reference if you have another mutable reference
    let xr = &x; // Immutable Reference
    let xrr = &mut x; // Mutable reference
    */
    {
        let xrr = &mut x;
        *xrr = 12; // mutate the value of x
    }

    // here we're borrowing the immutable reference of x. This won't compile unless the mutable reference mutates inside a block -- you can only have one and only one mutable reference
    println!("x is {}", x); 
}