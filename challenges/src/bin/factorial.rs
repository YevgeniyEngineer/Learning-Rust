/// Iterative factorial
///
/// ### Arguments
///
/// * `n` - A number to compute the factorial of
///
fn factorial_iterative(n: u64) -> u64 {
    let mut result = 1;

    for i in 1..=n {
        result *= i;
    }

    result
}

/*
Recursive factorial
*/
fn factorial_recursive(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial_recursive(n - 1)
    }
}

/*
Compile time factorial
*/
const fn compile_time_factorial(n: u64) -> u64 {
    let mut result = 1;
    let mut i = 1;

    while i <= n {
        result *= i;
        i += 1;
    }

    result
}

fn main() {
    let num = 5;
    println!(
        "The iterative factorial of {} is {}",
        num,
        factorial_iterative(num)
    );

    println!(
        "The recursive factorial of {} is {}",
        num,
        factorial_recursive(num)
    );

    const FACTORIAL_OF_5: u64 = compile_time_factorial(5);
    println!("The factorial of 5 is {}", FACTORIAL_OF_5);
}
