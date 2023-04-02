struct Point {
    x: f64,
    y: f64,
}

// Methods for Point struct
impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    // Takes an immutable reference to self and an immutable reference to another Point struct
    // The method does not modify internal states of any of the structs
    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    // Takes a mutable reference to self, and a scaling factor
    // Modifies Point in-place
    fn scale(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
    }
}

fn main() {
    let num1 = 10;
    let num2 = 20;

    // This function takes immutable references (const std::int32_t& in C++)
    let sum = add(&num1, &num2);
    println!("Sum of {} and {} is {}", num1, num2, sum);

    let mut num3 = 50;
    println!("Before increment: {}", num3);

    // This function takes a mutable reference (std::int32_t& in C++)
    increment(&mut num3);
    println!("After increment: {}", num3);

    let point1 = Point::new(0.0, 0.0);
    let mut point2 = Point::new(3.0, 4.0);

    let dist = point1.distance(&point2);
    println!("Distance between points: {:.2}", dist);

    point2.scale(2.0);
    println!("Scaled point2: ({:.2}, {:.2})", point2.x, point2.y);
}

fn add(a: &i32, b: &i32) -> i32 {
    a + b
}

fn increment(num: &mut i32) {
    *num += 1;
}
