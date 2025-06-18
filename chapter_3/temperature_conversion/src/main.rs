use std::io;

fn main() {
    let mut temp = String::new();
    let mut temp_type = String::new();

    println!("Enter a temperature number");
    io::stdin().read_line(&mut temp).expect("Failed to read line");

    println!("Enter a unit of temperature (F or C)");
    io::stdin().read_line(&mut temp_type).expect("Failed to read line");

    let temp_num = temp.trim().parse::<f32>().unwrap();

    // Thanks to https://stackoverflow.com/questions/25383488/how-to-match-a-string-against-string-literals
    match temp_type.as_str().trim() {
        "F" | "C" => {
            convert(temp_num, temp_type);
        },
        _ => println!("Unknown temperature type detected, unable to convert temperature")
    }
}

fn convert(temp_num: f32, temp_type: String) {
    let converted_temp;
    // There's probably an easier way to compare characters without all the trim statements I'm using
    let converted_type = if (temp_type.trim() == "C") { "F" } else { "C" };
    if converted_type.trim() == "C" {
        let coefficient = (5.0 / 9.0);
        converted_temp = ((temp_num - 32.0) * coefficient);
    } else {
        let coefficient = (9.0 / 5.0);
        converted_temp = (((temp_num as f32) * coefficient) + 32.0);
    }

    println!("Converted temperature: {} degrees {}",  converted_temp, converted_type);
}