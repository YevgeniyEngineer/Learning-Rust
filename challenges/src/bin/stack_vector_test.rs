use std::panic;

use challenges::stack_vector::StackVector;

fn main() {
    let mut stack_vector = StackVector::<i32, 10>::new();

    stack_vector.push(1);
    stack_vector.push(10);
    stack_vector.push(25);

    for elem in stack_vector.iter() {
        println!("{}", elem);
    }

    for elem in stack_vector.iter_rev() {
        println!("{}", elem);
    }

    println!(
        "Before clear: {:?}, size: {:?}",
        stack_vector,
        stack_vector.len()
    );

    let stack_vector_copy = stack_vector.clone();

    stack_vector.clear();

    println!(
        "After clear: {:?}, size: {:?}",
        stack_vector,
        stack_vector.len()
    );

    println!("Copy: {:?}", stack_vector_copy);

    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        for i in 0..15 {
            stack_vector.push(i);
            println!("Pushed {:?}", stack_vector[i as usize]);
        }
    }));

    assert!(result.is_err(), "Pushing beyond capacity should panic");
}
