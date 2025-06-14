fn main() {
    // Constants can be declared in any scope, including the global scope
    // Constants are valid for the entire life of a program, within the scope in which they are declared
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let x = 5;
    let x = x + 1; // This second declaration of x "shadows" the first, so the compiler will see this second variable instead of the first when "x" is referenced

    {
        // Shadowing in an inner scope doesn't affect outer scopes.
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Shadowing is different from mutating a variable.
    // With shadowing, we can change the type of the value and reuse the same name, since we're effectively re-declaring the variable
    // With a mutable variable, we can change the value, but NOT the type
    let spaces = "   ";
    let spaces = spaces.len();

    // Example of what happens if we try to change the type of a mutable variable - throws an error
    // let mut spaces = "   ";
    // spaces = spaces.len();
}