use std::fmt;
use crate::{Celsius, Fahrenheit, Kelvin};

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum TempFormat {
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
