use crate::SevenSegment;
use avr_device::atmega328p::Peripherals;
use core::ops::RangeInclusive;
use embedded_hal::digital::PinState;

#[allow(unused)]
pub fn run(dp: Peripherals) -> ! {
    let pins = arduino_hal::pins!(dp);

    let mut display = SevenSegment {
        // lowest 4
        b: pins.d4.into_opendrain(),
        a: pins.d5.into_opendrain(),
        f: pins.d6.into_opendrain(),
        g: pins.d7.into_opendrain(),
        // highest 4
        dp: pins.d8.into_opendrain_high(),
        c: pins.d9.into_opendrain(),
        d: pins.d10.into_opendrain(),
        e: pins.d11.into_opendrain(),
        // important since we light segments by opening for ground from 5V
        display_state: PinState::Low,
    };
    let ff_button = pins.d13.into_opendrain();
    let single_digits: RangeInclusive<u8> = 0..=9;
    let cycle = single_digits
        .clone()
        .chain(single_digits.clone().rev())
        .cycle();
    // back and forth, repeating the end values an extra time
    for next in cycle {
        display.set_digit(next).expect("handles all digits 0..=9");
        display.show_decimal(next == *single_digits.start() || next == *single_digits.end());
        if ff_button.is_high() {
            arduino_hal::delay_ms(70);
        } else {
            arduino_hal::delay_ms(250);
        }
    }
    unreachable!("iterator cycles forever")
}
