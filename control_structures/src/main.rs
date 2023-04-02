fn main() {
    let x = 5;

    if x < 10 {
        println!("x is less than 10");
    } else {
        println!("x is greater than or equal to 10");
    }

    let mut counter = 0;
    while counter < 5 {
        println!("counter: {}", counter);
        counter += 1;
    }

    for i in 1..6 {
        println!("i: {}", i);
    }
}
