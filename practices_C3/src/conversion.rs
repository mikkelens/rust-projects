use std::io;
use crate::conversion::TempFormat::{Celcius, Fahrenheit, Kelvin};

mod functions;

enum TempFormat {
    Celcius,
    Fahrenheit,
    Kelvin
}

pub fn run() {
    println!("Conversion tool!");
    println!("What do you want to convert TO?");

    // display all options
    let mut options: [TempFormat; 3] = [Celcius, Fahrenheit, Kelvin];

    // get first answer
    let mut convert_to: TempFormat = answer_format(&options);

    // reduce options
    let options = match convert_to {
        Celcius => {[Fahrenheit, Kelvin]}
        Fahrenheit => {[Celcius, Kelvin]}
        Kelvin => {[Celcius, Fahrenheit]}
    };

    // get second answer with limited options
    let mut convert_from: TempFormat = answer_format(&options);
}

fn answer_format(options: &[TempFormat]) -> TempFormat {

    return Celcius;
}