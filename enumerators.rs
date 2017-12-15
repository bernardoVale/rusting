enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    let player_direction:Direction = Direction::Down;

    match player_direction {
        Direction::Up => println!("Heading up"),
        Direction::Down => println!("Heading down"),
        Direction::Right => println!("Heading right"),
        Direction::Left => println!("Heading left"),
    }
}
