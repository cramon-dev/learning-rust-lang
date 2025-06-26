fn main() {
    let greeting = String::from("hello");
    // A "reference" is like a pointer in that it's an address we can follow to access data stored at that address
    // References let us refer to some value WITHOUT taking ownership of the value
    // The data is still owned by "greeting"; a reference is guaranteed to point to a valid value of a particular type for the life of that reference
    // Creating a reference is referred to as "borrowing"
    let len = calculate_length(&greeting);
    println!("The length of '{greeting}' is {len}");

    // References are immutable BY DEFAULT, just like variables
    // To make a mutable reference, we need a mutable variable, and then we need to add "&mut" to function signatures and calls for said reference
    let mut new_greeting = String::from("hello");
    change(&mut new_greeting);

    println!("{new_greeting}");

    // // ONLY one mutable reference to a value can be created
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    //
    // println!("{r1} {r2}");

    // // We can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous references:
    // let mut s = String::from("hello");
    //
    // {
    //     let r1 = &mut s;
    // } // r1 goes out of scope here, so a new reference can be created without problems
    //
    // let r2 = &mut s;

    // Reference scopes start from where they're declared and end where they're last used
    let mut another_greeting = String::from("hello");

    let r1 = &another_greeting; // no problem
    let r2 = &another_greeting; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point, and go out of scope

    let r3 = &mut another_greeting; // no problem
    r3.push_str(" and good day, world");
    println!("{r3}");

    // let dangling_ref = dangle();
}

// Rust prevents us from creating dangling references; the compiler will ensure that the data will not go out of scope before references to the data do
// fn dangle() -> &String { // returns a reference to a String
//     let s = String::from("hello"); // "s" is a new String on heap
//
//     &s // we return a reference to the String, "s"
// } // "s" goes out of scope, and is dropped, so its memory is freed

/*
    Some extra notes on mutable references:

    Although we can only have one mutable reference for a given value at any time, this actually benefits us
    because Rust can prevent "data races" at compile time.

    A "data race" is similar to a race condition and happens when these three behaviors occur:
      1. Two or more pointers access the same data at the same time
      2. At least one of the pointers is being used to write to the data
      3. There's no mechanism being used to synchronize access to the data

    Rust will not let us create both mutable and immutable references to the same value
    ```
        let r1 = &s; // fine
        let r2 = &s; // fine
        let r3 = &mut s; // NOT fine
        println!("{r1}, {r2}, {r3}");
    ```
*/

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Without using references, we would have to make sure we're returning the string we want to measure if we still want to use it in the calling function
// This is because we're moving "str" into this function, which will go out of scope and drop it before the calling function can use "str"
// fn calculate_length(str: String) -> (String, usize) {
//     let len = str.len();
//     (str, len)
// }

fn calculate_length(s: &String) -> (usize) {
    s.len()
} // "s" goes out of scope here, but because "s" doesn't have ownership of what it refers to, the value is not dropped

/*
             "s"
    ---------------------
    |   name  |  value  |
    |-------------------|
    |   ptr    |   o - - - -
    ---------------------   |
               - - - - - - -
               |
               v
             "s1"
    ---------------------
    |   name  |  value  |
    |-------------------|
    |   ptr    |   o - - - -
    |   len    |   5    |  | // length = how much memory in bytes the contents of String are currently using
    | capacity |   5    |  | // capacity = total amount of memory in bytes that String has received from allocator
    ---------------------  |
           - - - - - - - - -
           |
           v
      (stored on heap)
    ---------------------
    |  index  |  value  |
    |-------------------|
    |     0    |   h    |
    |     1    |   e    |
    |     2    |   l    |
    |     3    |   l    |
    |     4    |   o    |
    ---------------------
 */