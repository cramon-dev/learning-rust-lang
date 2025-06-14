fn main() {
    // In situations like below where more than one type is possible for a statement,
    // the compiler needs a type annotation to know what to expect.
    // let guess = "42".parse().expect("Not a number!"); // <-- will throw an error
    // println!("{}", guess);

    // ===== Scalar types =====
    // Rust has four scalar types: integers, floating-point numbers, booleans, and characters

    // ===== Integer =====
    // A number without a fractional component (a whole number)
    // Integers can either be signed or unsigned, and an explicit size in bits.
    {
        let small_number = i8::MAX;
        println!("Biggest number possible with a signed 8-bit int: {}", small_number);
        
        let small_unsigned_number = u8::MAX;
        println!("Biggest number possible with an unsigned 8-bit int: {}", small_unsigned_number);

        let unsigned_number = i32::MAX;
        println!("Biggest number possible with an unsigned 32-bit int: {}", unsigned_number);

        let colossal_number = i128::MAX;
        println!("Biggest number possible with a signed 128-bit int: {}", colossal_number);

        // "isize" and "usize" are special cases: depends on the architecture of the computer the program is running on 
        let arch_signed_number = isize::MAX;
        println!("Biggest number possible with my computer's architecture (should match 64 bit max): {}", arch_signed_number);
        println!("Biggest number possible with a signed 64-bit int: {}", i64::MAX);

        // You can use "_" as a visual separator in number literals to make them easier to read
        println!("This should print the number without '_' characters: {}", 13_034_431);

        // You can also prefix number literals if you want to specify a different base, like hex
        println!("Hex number should print 15: {}", 0xF);
        println!("Octal number should print 63: {}", 0o77);
    }

    // In case of integer overflow:
    // If running in debug, panic will occur
    // If running in release, overflow wrapping will occur (ex. adding 1 to a u8 number with the value 255 will cause it to become 0)
    println!("Saturating the 8-bit unsigned max (254) with 1: {}", u8::saturating_add(u8::MAX, 1));

    // ===== Floating-point numbers =====
    // 64-bit is the default on most computers nowadays since performance hits using 64 bit are negligable
    // ALL floating-point types are signed
    {
        let floating_64_number = 2.0; // type is "f64"
        let floating_32_number: f32 = 1.64; // type is "f32"
    }

    // ===== Numeric operations =====
    // Standard operations are supported (addition, multiplication, subtraction, division)
    // Special case: dividing integers truncates towards zero to the nearest int
    {
        let truncated = -5 / 3; // Results in -1
        let bigger_truncated_number = 9 / 2; // Results in 4
        println!("-5 / 3 in Rust = {}", truncated);
        println!("-9 / 2 in Rust = {}", bigger_truncated_number);
    }

    // ===== Boolean type =====
    // Nothing special to note: one byte in size and two possible values: true and false

    // ===== Character type =====
    // Character literals are specified with single quotes
    // Characters are four bytes in size and rep a Unicode scalar value, which can rep much more than ASCII
    {
        let c = 'z';
        let z: char = 'ℤ'; // with explicit type annotation
        let heart_eyed_cat = '😻';
    }

    // ===== Compound types =====
    // Used to group multiple values into one type. Two types exist in Rust: tuple and array

    // ===== Tuples =====
    // General method of grouping multiple values with varying types into one compound type
    // Fixed length once declared; they cannot be shrunk or grown afterwards
    // First index in a tuple is zero (tuples are zero-based)
    {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        // We can utilize pattern matching to destructure a tuple when we want to get individual values from the tuple
        let (a, b, c) = tup; // a = 500, b = 6.4, c = 1
        // We can also use dot notation to access values from within the tuple
        let five_hundred = tup.0;
        let six_point_four = tup.1;
        let one = tup.2;
        // Tuple without any values is referred to as "unit"
        // "unit" and its type are written as "()" and represent an empty value or empty return type
        // Expressions implicitly return a unit value if they don't return any other value
    }

    // ===== Arrays =====
    // Arrays in Rust have a fixed length
    // Every element in an array MUST be of the same type
    // Arrays are useful when we need our data to be allocated on stack rather than heap
    // Vectors are a similar collection type which ARE allowed to grow or shrink in size
    // If unsure which collection type to use, prefer a vector
    {
        let numbers = [1, 2, 3, 4, 5];
        let five_numbers: [i32; 5] = [6, 7, 8, 9, 10]; // Another way of declaring the specific type and length of an array
        // let values = [1, 2, "3", 4, 5]; // <-- throws a "mismatched types" error
        // We can initialize arrays to contain the same value for each element
        let same_numbers = [3; 5]; // <-- [3, 3, 3, 3, 3]
        // Accessing an array is simple: use the index of the value you're looking for
        let one = numbers[0];

        // Accessing an invalid element of the array (out-of-bounds) will result in a panic as seen below
        {
            let a = [1, 2, 3, 4, 5];
            println!("Please enter an array index.");

            let mut index = String::new();

            std::io::stdin()
                .read_line(&mut index)
                .expect("Failed to read line");

            let index: usize = index
                .trim()
                .parse()
                .expect("Index entered was not a number");

            let element = a[index];

            println!("The value of the element at index {index} is: {element}");
        }
    }

}
