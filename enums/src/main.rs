fn main() {
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    let movement = Direction::Up;

    match movement {
        Direction::Up => println!("Moving up!"),
        Direction::Down => println!("Moving down!"),
        Direction::Left => println!("Moving left!"),
        Direction::Right => println!("Moving right!"),
    }
}
