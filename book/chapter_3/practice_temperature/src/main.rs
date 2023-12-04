use std::io::{stdin, stdout, Write};
use temp_formats::TempFormat;
use temp_formats::TempFormat::{Celsius, Fahrenheit, Kelvin};

mod temp_formats;
mod functions;

fn main() {
    println!("\n--- TEMPERATURE CONVERSION TOOL ---"); // start
    loop {
        println!(); // spacing
        calculation(); // calculator
        println!(); // spacing

        println!("Do you want to convert another temperature? (Y/N)");
        if get_input_string(">").to_lowercase().contains("n") {
            break;
        }
    }
} // end

fn calculation() {
    let mut options = vec![Celsius, Fahrenheit, Kelvin];

    // first format choice
    println!("What do you want to convert *from*?");
    // display all options
    println!("All formats include {}, {} and {}.", options[0], options[1], options[2]);
    let convert_from: TempFormat = choose_format(&options, "FROM:");

    // reduce choices
    options.retain(|f| *f != convert_from);

    // second format choice (with limited options)
    println!("What do you want to convert *to*?");
    // display reduced options
    println!("Remaining formats are {} and {}.", options[0], options[1]);
    let convert_to: TempFormat = choose_format(&options, "TO:");

    // value input
    println!("What is the temperature (in {}) you want to convert to {}?", convert_from, convert_to);
    let value = set_value(format!("{}:", convert_from).as_str());

    println!("You converted {} {} to {}...", value, convert_from, convert_to);

    // then use correct function to calculate
    let result = functions::convert_value(convert_from, convert_to, value);
    println!("The result is {:.3} {}!", result, convert_to);
}

fn choose_format(options: &Vec<TempFormat>, cursor_prompt: &str) -> TempFormat {
    // loop until a valid answer is given
    loop {
        let choice = match get_input_string(cursor_prompt).to_lowercase().trim() {
            "c" | "celsius" => Celsius,
            "f" | "fahrenheit" => Fahrenheit,
            "k" | "kelvin" => Kelvin,
            _ => {
                println!("You need to choose a valid unit of temperature.");
                continue;
            }
        };

        // check if legal conversion
        for _option in options {
            if &choice == _option {
                return choice;
            }
        }
        println!("You can only convert from one unit to a different unit.")
        // try again
    }
}

fn get_input_string(cursor_prompt: &str) -> String {
    print!("{} ", cursor_prompt);
    stdout().flush().expect("Could not flush.");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    return input;
}

fn set_value(cursor_prompt: &str) -> f64 {
    // loop until valid answer is given
    loop {
        let answer: f64 = match get_input_string(cursor_prompt).trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You have to use a number.");
                continue;
            }
        };
        return answer;
    }
}