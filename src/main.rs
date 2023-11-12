use serde_json;
use std::env;

// Available if you need it!
// use serde_bencode

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    if encoded_value.contains(":") {
        // If the number string before the separator is not a real number this panic will handle it.
        if !encoded_value.chars().next().unwrap().is_digit(10) {
            eprintln!("Unhandled encoded value: {}", encoded_value);
            std::process::exit(1);
        }

        // Retrieve the index for the separator ":"
        let colon_index = encoded_value.find(':').unwrap();

        // Get the string before the separator ":"
        let number_string = &encoded_value[..colon_index];

        // Parse the number string into a i64
        let number = number_string.parse::<i64>().unwrap();

        // Retrieve the string based on the index of separator, here we trust that the input is the
        // correctly length (`number`) of the string (`string).
        let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];

        // Return the string value
        return serde_json::Value::String(string.to_string());
    } else if encoded_value.contains("i") {
        // Get the number inside the `i` and `e` (i.e. i52e)
        let number_string = &encoded_value[1..&encoded_value.len() - 1];

        // Parse the string number into a real number
        let number = number_string.parse::<i64>().unwrap();

        // Return the number value
        return serde_json::Value::Number(number.into());
    } else {
        todo!("we don't know how to handle {encoded_value}");
    }
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    } else {
        eprintln!("unknown command: {}", args[1]);
        std::process::exit(1);
    }
}
