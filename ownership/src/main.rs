fn main() {
    let s = String::from("hello"); // s comes into scoep

    let x = 5; // x comes into scope
    other_func(x); // i32 is a Copy, so it's still okay in this scope
    println!("After some_int called: {}", x);

    let s1 = some_func(s); // s's value moves into the function - takes ownership
    println!("{s1}");

    // While this works, taking ownership and then returning ownership with every function is a bit tedious.
    // What if we want to let a function use a value but not take ownership? It’s quite annoying that anything we
    // pass in also needs to be passed back if we want to use it again, in addition to any data resulting from the
    // body of the function that we might want to return as well.
    let message = String::from("Some random message");
    let len = calculate_length(&message);
    println!("Length of '{message}' is {len}");

    // Mutable reference
    let mut missing = String::from("Hello");
    change(&mut missing);
    println!("{missing}");
}

fn some_func(some_string: String) -> String {
    println!("Inside the scope of some_func: {}", some_string);
    return some_string; // Move back into scope where func was called
}

fn other_func(some_int: i32) {
    println!("Inside scope of some_int: {}", some_int);
}

fn calculate_length(message: &String) -> usize {
    return message.len();
}

fn change(message: &mut String) {
    message.push_str(" world");
}
