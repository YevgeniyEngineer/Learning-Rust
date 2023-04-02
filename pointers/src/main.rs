use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Mutable unique pointer in rust
    let mut num = Box::new(5);
    println!("num = {}", num);
    println!("Memory address of num: 0x{:x}", &*num as *const _ as usize);

    // 1) Pass by value (ownership transfer)
    num = increment_boxed_value(num);
    println!("Incremented by value: {}", num);
    println!("Memory address of num: 0x{:x}", &*num as *const _ as usize);

    // 2) Pass by mutable reference (no ownership transfer)
    increment_boxed_value_by_ref(&mut num);
    println!("Incremented the mutable reference: {}", num);
    println!("Memory address of num: 0x{:x}", &*num as *const _ as usize);

    // Shared pointer
    // Only allows read-only sharing
    let original = Rc::new(10);
    let clone1 = Rc::clone(&original);
    let clone2 = Rc::clone(&original);
    println!(
        "original: {}, clone1: {}, clone2: {}",
        original, clone1, clone2
    );
    print_shared_value(&original);

    // Mutable memory location that enforces borrowing rules at runtime instead of compile time
    // Allows to mutate data through an immutable reference
    let shared = Rc::new(RefCell::new(15));
    println!("Shared mutable state before incrementing: {:?}", shared);
    {
        let mut shared_mut = shared.borrow_mut();
        *shared_mut += 1;
    }
    println!("Shared mutable state after incrementing: {:?}", shared);
}

// Increment the value inside of a Box and return a new Box
fn increment_boxed_value(mut boxed_num: Box<i32>) -> Box<i32> {
    // Dereference pointer
    *boxed_num += 1;

    // Return the value (ownership returned back)
    boxed_num
}

// Increment a value inside of a Box using a mutable reference
fn increment_boxed_value_by_ref(boxed_num: &mut Box<i32>) {
    // Dereference the mutable reference and increment
    // The first dereference gets from the mutable reference (&mut Box<i32>) to (Box<i32>)
    // The second dereference gets from (Box<i32>) to (i32) value store by the pointer
    **boxed_num += 1;
}

// Takes const reference to shared pointer
fn print_shared_value(shared_num: &Rc<i32>) {
    // Dereference the shared pointer to access the value inside it
    println!("Shared value: {}", *shared_num);
}
