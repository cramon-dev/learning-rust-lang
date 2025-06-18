use std::path::Component::ParentDir;

fn main() {
    let number = 3;

    if number < 5 {
        println!("number is LESS than 5");
    } else {
        println!("number is GREATER than or equal to 5");
    }

    // Unlike in JavaScript, Rust doesn't try to convert non-Boolean types to Booleans
    // if number {
    //     println!("number is three");
    // }

    if number != 0 {
        println!("number was something other than zero");
    }

    let second_number = 6;
    if second_number % 4 == 0 {
        println!("second number is divisible by 4");
    } else if second_number % 3 == 0 {
        println!("second number is divisible by 3");
    } else if second_number % 2 == 0 {
        println!("second number is divisible by 2");
    } else {
        println!("second number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    // Because "if" is an expression, we can use "if" blocks to assign outcomes to variables
    let third_number = if condition { 5 } else { 6 };
    println!("The value of third_number is: {}", third_number);

    // This expression is invalid because both arms of the "if" block have to return the same type
    // let second_condition = true;
    // let number = if second_condition { 5 } else { "six" };

    println!("==========");
    loops();
    println!("==========");
    while_loops();
    println!("==========");
    while_loops_array();
    println!("==========");
    for_loops_array();
    println!("==========");
    for_loops_numbers();
    println!("==========");
    for_loops_strings();
}

fn loops() {
    // // "loop" will continuously execute the code in the given block unless interrupted somehow
    // loop {
    //     println!("Again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }

        // We can use "return" from inside loops, but this will exit the current function
    };

    println!("The result of the first loop is {}", result);

    let mut second_counter = 0;
    // "break" and "continue" apply to the innermost loop
    // If we have nested loops, we can label loops then use said keywords to specify
    // that those keywords apply to the labeled loop rather than the innermost loop
    // Labels MUST begin with a single quote
    'counting_up: loop {
        println!("count = {second_counter}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if second_counter == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        second_counter += 1;
    }

    println!("End second counter = {second_counter}");
}

fn while_loops() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    
    println!("Goodbye, while loop!!");
}

fn while_loops_array() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value of a[{index}] is: {}", a[index]);
        index += 1;
    }
}

fn for_loops_array() {
    let a= [10, 20, 30, 40, 50];
    // To my understanding, the expression following "in" just has to be of an iterable type
    for element in a {
        println!("The value is: {}", element);
    }
}

fn for_loops_numbers() {
    for number in (1..4).rev() {
        println!("{number}...");
    }
    
    println!("LIFTOFF!!!!!");
}

fn for_loops_strings() {
    let characters = "abcdefg";
    for char in characters.chars() {
        println!("{char}");
    }
}