use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();

    print_labeled_measurement(5, 'h');

    let x = five();
    println!("The value of x is: {x}");

    let x = sum_two_numbers(x, 20);
    println!("The value of x is now: {x}");

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    loopy_loop();

    let celcius = fahrenheit_to_celcius(102.0);
    println!("102 Fahrenheit in Celcius is: {celcius}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn sum_two_numbers(x: i32, y: i32) -> i32 {
    x + y
}

fn loopy_loop() {
    let mut count = 0;

    // Labeled loop that can be called within nested loop
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}

fn fahrenheit_to_celcius(value: f64) -> String {
    let result = (value - 32.0) * 5.0 / 9.0;
    return format!("{:.2}", result);
}
