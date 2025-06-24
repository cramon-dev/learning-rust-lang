fn main() {
    string_literal_scope();
    string_object_scope();
    assign_new_value();
    clone_heap_data();
    clone_stack_data();
}

fn string_literal_scope() {
    // println!("{s}"); // "s" isn't declared before this block, so it doesn't exist
    {
        let s = "Hello"; // "s" is a valid reference for the rest of this block scope
        println!("{s}");
        // do other things here with s...
    }
    // println!("{s}"); // We're out of scope now, so "s" isn't accessible
}

fn string_object_scope() {
    // Strings created from this "String" type *are* mutable unlike string literals
    let mut s = String::from("Hello"); // This will allocate some memory on the heap
    s.push_str(", world!");
    println!("{s}");
    // When "s" goes out of scope, Rust will call "drop" for us to dispose of the object
}

fn assign_new_value() {
    let mut s = String::from("hello");
    s = String::from("ahoy"); // When assigning "s" to a new value, Rust calls "drop" and frees the original memory immediately

    println!("{s}, world!");
}

fn clone_heap_data() {
    // If we want to deeply copy heap data and not just stack data, we can use the "clone" method
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}

fn clone_stack_data() {
    // Types such as ints that have a known size at compile time are stored on stack, so copies of the actual values are quick to make
    // Rust has a special annotation called the "Copy" trait that we can place on types that are stored on stack
    // If a type implements the "Copy" trait, variables that use it do not move, but rather are copied, making them still valid after assignment to another variable
    // Rust won't let us annotate a type with "Copy" if the type, or any of its parts, has implemented the "Drop" trait
    // NOTHING that requires allocation or is some form of resource can implement "Copy"
    // In general, any group of simple scalar values can implement "Copy", such as:
    // - All integer types
    // - The Boolean type
    // - All floating point types
    // - Character type
    // - Tuples, if they only contain types that also implement "Copy"
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");
}

// let s1 = String::from("hello");
// let s2 = s1; // when assigning s1 to s2, String data is copied, meaning the pointer, length, and capacity are copied
// Since both s1 and s2 point to the same data on heap this could potentially result in a "double free" error when both s1 and s2 go out of scope
// Freeing memory twice can lead to memory corruption, and a potential memory safety bug
// To address this, after the line "s2 = s1", Rust considers s1 as no longer valid
// If we try to use the "s1" variable after this line, we'll get a "borrow of moved value" error on compile because Rust stops us from using invalidated refs
// Because Rust invalidates the first variable, this isn't called a shallow copy, but rather, a "move" (s1 was "moved" into s2)
// Rust will NEVER automatically create "deep" copies of our data
/*
            s1 (stored on stack)
   ---------------------
   |   name  |  value  |
   |-------------------|
   |   ptr    |   o - - - -
   |   len    |   5    |  | // length = how much memory in bytes the contents of String are currently using
   | capacity |   5    |  | // capacity = total amount of memory in bytes that String has received from allocator
   ---------------------  |
             - - - - - - -
             |
             v  (stored on heap)
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