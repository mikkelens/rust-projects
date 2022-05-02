const C_TO_F_SCALAR: f64 = 1.8;
const C_TO_F_OFFSET: f64 = 32.0;
const C_TO_K_OFFSET: f64 = 273.15;
const F_TO_K_OFFSET: f64 = 459.67;

// from fahrenheit
pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - C_TO_F_OFFSET) / C_TO_F_SCALAR
}
pub fn fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
    (fahrenheit + F_TO_K_OFFSET) / C_TO_F_SCALAR
}

// from celsius
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * C_TO_F_SCALAR + C_TO_F_OFFSET
}
pub fn celsius_to_kelvin(celsius:f64) -> f64 {
    celsius + C_TO_K_OFFSET
}

// from kelvin
pub fn kelvin_to_celsius(kelvin:f64) -> f64 {
    kelvin - C_TO_K_OFFSET
}
pub fn kelvin_to_fahrenheit(kelvin:f64) -> f64 {
    kelvin * C_TO_F_SCALAR - F_TO_K_OFFSET
}
