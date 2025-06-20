use std::io;

fn main() {
    let verse_num: i32 = parse_input();
    sing_carol(verse_num);
}

fn parse_input() -> i32 {
    println!("To what verse would you like me to sing your Christmas carol? (1-12)");

    let mut input = String::new();
    loop {
        io::stdin().read_line(&mut input).unwrap();
        let valid_num = input.trim().parse::<i32>().is_ok();

        if !valid_num {
            println!("Please enter a number from 1 to 12.");
            continue;
        }

        let parsed_num = input.trim().parse::<i32>().unwrap();

        if parsed_num >= 1 && parsed_num <= 12 {
            break parsed_num;
        } else {
            println!("Please enter a number from 1 to 12.");
        }
    }
}

fn sing_carol(verse_num: i32) {
    // Last number in a range expression in a for loop is exclusive
    for day in 1..verse_num + 1 {
        if day == 1 {
            println!("On the first day of Christmas, my true love gave to me...");
            println!("A partridge in a pear tree!\n");
            continue;
        }

        println!("On the {} day of Christmas, my true love gave to me...", generate_verse_number(day));

        for i in (1..day + 1).rev() {
            println!("{}", generate_verse(i));
        }

        println!();
    }
}

fn generate_verse_number(day: i32) -> String {
    match day {
        2 => String::from("second"),
        3 => String::from("third"),
        4 => String::from("fourth"),
        5 => String::from("fifth"),
        6 => String::from("sixth"),
        7 => String::from("seventh"),
        8 => String::from("eighth"),
        9 => String::from("ninth"),
        10 => String::from("tenth"),
        11 => String::from("eleventh"),
        12 => String::from("twelfth"),
        _ => String::from("first")
    }
}

fn generate_verse(day: i32) -> String {
    match day {
        2 => String::from("Two turtle doves"),
        3 => String::from("Three French hens"),
        4 => String::from("Four calling birds"),
        5 => String::from("Five golden rings"),
        6 => String::from("Six geese a-laying"),
        7 => String::from("Seven swans a-swimming"),
        8 => String::from("Eight maids a-milking"),
        9 => String::from("Nine ladies dancing"),
        10 => String::from("Ten lords a-leaping"),
        11 => String::from("Eleven pipers piping"),
        12 => String::from("Twelve drummers drumming"),
        _ => String::from("And a partridge in a pear tree!")
    }
}