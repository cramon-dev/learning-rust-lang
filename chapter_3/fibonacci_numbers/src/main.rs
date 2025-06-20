use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let nth_fibonacci_num = input.trim().parse::<i32>().unwrap();

    let result = generate(nth_fibonacci_num);

    println!("Fibonacci number nth index {nth_fibonacci_num}: {result}");
}

fn generate(nth_fibonacci_num: i32) -> i32 {
    match nth_fibonacci_num {
        0 => 0,
        1 => 1,
        _ => generate(nth_fibonacci_num - 1) + generate(nth_fibonacci_num - 2)
    }
}