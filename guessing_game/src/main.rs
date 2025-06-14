use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number:");

    let secret_number = fastrand::u32(1..100);
    
    loop {
        // Create a mutable variable, since variables are immutable by default
        // :: indicates that new is an associated function of String, or simply a function implemented on a type
        let mut guess = String::new();

        // Alternatively, could deeply import from std library without importing all of std::io, like so: "std::io::stdin"
        io::stdin()
            .read_line(&mut guess) // read_line expects a reference to a mutable string. Also, references are immutable by default
            .expect("Failed to read line"); // Expect is a function made available on Result types (which are enums)
    
        // Shadow and annotate the variable with the specific number type that we want parsed from the inputted string
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
