#![no_std]
#![no_main]

extern crate panic_halt;

#[allow(unused)]
const GIF_DATA: &[u8] = include_bytes!("smol_amogus.gif");

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();

    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}
