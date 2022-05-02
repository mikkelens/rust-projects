use std::{fmt};
use std::io::{stdin, stdout, Write};
use crate::conversion::functions::{celsius_to_fahrenheit, celsius_to_kelvin, fahrenheit_to_celsius, fahrenheit_to_kelvin, kelvin_to_celsius, kelvin_to_fahrenheit};
use crate::conversion::TempFormat::{Celsius, Fahrenheit, Kelvin};

mod functions;

#[derive(PartialEq, Eq, Copy, Clone)]
enum TempFormat {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl fmt::Display for TempFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Celsius => write!(f, "Celsius"),
            Fahrenheit => write!(f, "Fahrenheit"),
            Kelvin => write!(f, "Kelvin"),
        }
    }
}

pub fn run() {
    println!("--- TEMPERATURE CONVERSION TOOL ---");
    loop {
        calculation_event();

        println!("\nDo you want to convert another temperature? (Y/N)");
        if get_input().to_lowercase().contains("n") {
            break;
        }
    }

}

fn calculation_event() {

    let mut options = vec![Celsius, Fahrenheit, Kelvin];
    // display all options
    println!("\nAll formats: {}, {} and {}.", options[0], options[1], options[2]);

    // first format choice
    println!("What to convert *FROM*?");
    let convert_from: TempFormat = choose_format(&options);

    // display reduced options
    options.retain(|f| *f != convert_from);
    println!("Remaining formats: {} and {}.", options[0], options[1]);

    // second format choice (with limited options)
    println!("What to convert *TO*?");
    let convert_to: TempFormat = choose_format(&options);

    // value input
    println!("What is the temperature you want to calculate?");
    let value = set_value();

    println!("You converted {} {} to {}...", value, convert_from, convert_to);

    // then use correct function to calculate
    let result = convert_value(convert_from, convert_to, value);
    println!("Result is {}.", result);
}

fn choose_format(options: &Vec<TempFormat>) -> TempFormat {
    // loop until a valid answer is given
    loop {
        let choice = match get_input().to_lowercase().trim() {
            "c" | "celsius" => Celsius,
            "f" | "fahrenheit" => Fahrenheit,
            "k" | "kelvin" => Kelvin,
            _ => {
                println!("You need to choose either Celsius, Fahrenheit or Kelvin");
                continue;
            }
        };

        // check if legal conversion
        for _option in options {
            if &choice == _option {
                return choice;
            }
        }
        println!("You can only convert from one unit to another.")
        // try again
    }
}

fn get_input() -> String {
    print!("> ");
    stdout().flush().expect("Could not flush.");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    return input;
}

fn set_value() -> f64 {
    // loop until valid answer is given
    loop {
        let answer: f64 = match get_input().trim().parse() {
                Ok(num) => num,
                Err(_) => {
                println!("You have to use a number.");
                continue;
            }
        };
        return answer;
    }
}

fn convert_value(convert_from: TempFormat, convert_to: TempFormat, value: f64) -> f64 {
    return match (convert_from, convert_to) {
        (Celsius, Fahrenheit) => celsius_to_fahrenheit(value),
        (Celsius, Kelvin) => celsius_to_kelvin(value),
        (Fahrenheit, Celsius) => fahrenheit_to_celsius(value),
        (Fahrenheit, Kelvin) => fahrenheit_to_kelvin(value),
        (Kelvin, Celsius) => kelvin_to_celsius(value),
        (Kelvin, Fahrenheit) => kelvin_to_fahrenheit(value),
        _ => unreachable!()
    };
}