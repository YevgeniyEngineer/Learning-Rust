fn main() {
    // Tuple
    let point = (1, 2);
    println!("Point: ({}, {})", point.0, point.1);

    // Array
    let arr = [1, 2, 3, 4, 5];
    println!("First element: {}", arr[0]);

    // Struct
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 3, y: 4 };
    println!("Point: ({}, {})", p.x, p.y);
}
