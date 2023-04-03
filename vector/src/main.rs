use rand;
use std::iter::repeat_with;

fn main() {
    // Create a new vector of elements of type i32
    let mut numbers: Vec<i32> = Vec::new();

    // Add elements
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    // Print original vector
    println!("Original vector: {:?}", numbers);

    // Access elements using indexing
    println!("Second element: {}", numbers[1]);

    // Access elements using `get` method (returns an Option)
    match numbers.get(1) {
        Some(number) => println!("Second element: {}", number),
        None => println!("Elements not found"),
    }

    // Remove the last element and retrieve its value
    let last_number = numbers.pop();
    println!("Popped last number: {:?}", last_number);

    // Iterate over the vector elements
    println!("Iterating over the vector elements:");
    for number in &numbers {
        println!("{}", number);
    }

    // Modify vector elements using mutable references
    for number in numbers.iter_mut() {
        *number *= 2;
    }

    println!("Modified vector: {:?}", numbers);

    // Create a vector of given length with a default value
    let zeroes = vec![0; 5];
    println!("Vector of zeroes: {:?}", zeroes);

    // Create a vector of given length with a closure generated value
    let random_numbers: Vec<i32> = repeat_with(|| rand::random::<i32>()).take(5).collect();
    println!("Vector of random numbers: {:?}", random_numbers);
}
