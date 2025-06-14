// Rust is an expression-based language
// Function bodies are composed of a series of statements optionally ending in an expression
// Statements are instructions that perform some action and do not return a value
// Expressions evaluate to a resultant value
fn main() {
    let x = 3; // This is a statement
    // let x = (let y = 6); // This is invalid because a statement doesn't return a value
    let y = { // A scope block created with curly braces IS an expression
        let x = 3;
        x + 1 // Expressions DO NOT INCLUDE ending semicolons; adding one turns it into a statement
    };
    another_function(x); // Calling a function is an expression
    print_labeled_measurement(y, 'h');
    let z = five();
    println!("The value of z is: {}", z);
}

// Snake casing is the standard for variable and function naming in Rust
// Order of functions in file doesn't matter in Rust; we could push this function before the main declaration if we wanted
fn another_function(x: i32) {
    println!("The value of x is: {}", x); // Calling a macro is an expression
}

// Note that types MUST be declared for each parameter in a function. This is a design decision in the language itself
fn print_labeled_measurement(num: i32, unit_label: char) {
    println!("The measurement is: {num}{unit_label}");
}

fn five() -> i32 {
    5 // "return" keyword can be used to return early from a function, but is not necessary
}