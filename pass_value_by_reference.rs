struct Color {
    red: u8,
    green: u8,
    blue: u8
}

fn main() {
    let bg = Color {red: 255, green: 70, blue: 15 }; 
    print_color(&bg);
}

fn print_color(c: &Color) {
    println!("Color - R:{} G:{} B:{}", c.red, c.blue, c.green);
}