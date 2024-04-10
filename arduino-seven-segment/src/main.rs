#![no_std]
#![no_main]

/*
 * For examples (and inspiration), head to
 *
 *     https://github.com/Rahix/avr-hal/tree/main/examples
 *
 * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
 * for a different board can be adapted for yours.  The Arduino Uno currently has the most
 * examples available.
 */

use arduino_hal::port::mode::OpenDrain;
use arduino_hal::port::{Pin, PinOps};
use core::ops::RangeInclusive;
use embedded_hal::digital::{OutputPin, PinState};

mod panic;

/// Representing a physical seven segment LED display.
/// For diagram of what the labels mean, see link:
/// https://components101.com/sites/default/files/component_pin/7-segment-display-pin-diagr_0.png
/// Different generics are needed because each pin is a different concrete type.
struct SevenSegment<
    A: PinOps,
    B: PinOps,
    C: PinOps,
    D: PinOps,
    E: PinOps,
    F: PinOps,
    G: PinOps,
    DP: PinOps,
> {
    a: Pin<OpenDrain, A>,   // top bar
    b: Pin<OpenDrain, B>,   // upper right
    c: Pin<OpenDrain, C>,   // lower right
    d: Pin<OpenDrain, D>,   // bottom bar
    e: Pin<OpenDrain, E>,   // lower left
    f: Pin<OpenDrain, F>,   // upper left
    g: Pin<OpenDrain, G>,   // middle bar
    dp: Pin<OpenDrain, DP>, // dot in the bottom right corner
    display_state: PinState,
}
#[derive(Debug)]
enum DigitError {
    ValueTooBig,
}
impl<A: PinOps, B: PinOps, C: PinOps, D: PinOps, E: PinOps, F: PinOps, G: PinOps, DP: PinOps>
    SevenSegment<A, B, C, D, E, F, G, DP>
{
    /// Sets digit to a specific value.
    /// Does not touch the decimal point (`DP`).
    fn set_digit(&mut self, num: u8) -> Result<(), DigitError> {
        let on = self.display_state;
        let off = !on;
        match num {
            0 => {
                self.a.set_state(on).unwrap(); // set_state error type is `Infallible`
                self.b.set_state(on).unwrap();
                self.c.set_state(on).unwrap();
                self.d.set_state(on).unwrap();
                self.e.set_state(on).unwrap();
                self.f.set_state(on).unwrap();
                self.g.set_state(off).unwrap();
            }
            1 => {
                self.a.set_state(off).unwrap(); // set_state error type is `Infallible`
                self.b.set_state(on).unwrap();
                self.c.set_state(on).unwrap();
                self.d.set_state(off).unwrap();
                self.e.set_state(off).unwrap();
                self.f.set_state(off).unwrap();
                self.g.set_state(off).unwrap();
            }
            2 => {
                self.a.set_state(on).unwrap(); // set_state error type is `Infallible`
                self.b.set_state(on).unwrap();
                self.c.set_state(off).unwrap();
                self.d.set_state(on).unwrap();
                self.e.set_state(on).unwrap();
                self.f.set_state(off).unwrap();
                self.g.set_state(on).unwrap();
            }
            3 => {
                self.a.set_state(on).unwrap(); // set_state error type is `Infallible`
                self.b.set_state(on).unwrap();
                self.c.set_state(on).unwrap();
                self.d.set_state(on).unwrap();
                self.e.set_state(off).unwrap();
                self.f.set_state(off).unwrap();
                self.g.set_state(on).unwrap();
            }
            4 => {
                self.a.set_state(off).unwrap(); // set_state error type is `Infallible`
                self.b.set_state(on).unwrap();
                self.c.set_state(on).unwrap();
                self.d.set_state(off).unwrap();
                self.e.set_state(off).unwrap();
                self.f.set_state(on).unwrap();
                self.g.set_state(on).unwrap();
            }
            5 => {
                self.a.set_state(on).unwrap(); // set_state error type is `Infallible`
                self.b.set_state(off).unwrap();
                self.c.set_state(on).unwrap();
                self.d.set_state(on).unwrap();
                self.e.set_state(off).unwrap();
                self.f.set_state(on).unwrap();
                self.g.set_state(on).unwrap();
            }
            6 => {
                self.a.set_state(on).unwrap(); // set_state error type is `Infallible`
                self.b.set_state(off).unwrap();
                self.c.set_state(on).unwrap();
                self.d.set_state(on).unwrap();
                self.e.set_state(on).unwrap();
                self.f.set_state(on).unwrap();
                self.g.set_state(on).unwrap();
            }
            7 => {
                self.a.set_state(on).unwrap(); // set_state error type is `Infallible`
                self.b.set_state(on).unwrap();
                self.c.set_state(on).unwrap();
                self.d.set_state(off).unwrap();
                self.e.set_state(off).unwrap();
                self.f.set_state(off).unwrap();
                self.g.set_state(off).unwrap();
            }
            8 => {
                self.a.set_state(on).unwrap(); // set_state error type is `Infallible`
                self.b.set_state(on).unwrap();
                self.c.set_state(on).unwrap();
                self.d.set_state(on).unwrap();
                self.e.set_state(on).unwrap();
                self.f.set_state(on).unwrap();
                self.g.set_state(on).unwrap();
            }
            9 => {
                self.a.set_state(on).unwrap(); // set_state error type is `Infallible`
                self.b.set_state(on).unwrap();
                self.c.set_state(on).unwrap();
                self.d.set_state(on).unwrap();
                self.e.set_state(off).unwrap();
                self.f.set_state(on).unwrap();
                self.g.set_state(on).unwrap();
            }
            10.. => return Err(DigitError::ValueTooBig),
        }
        Ok(())
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut display = SevenSegment {
        // lowest 4
        b: pins.d4.into_opendrain(),
        a: pins.d5.into_opendrain(),
        f: pins.d6.into_opendrain(),
        g: pins.d7.into_opendrain(),
        // highest 4
        dp: pins.d8.into_opendrain(),
        c: pins.d9.into_opendrain(),
        d: pins.d10.into_opendrain(),
        e: pins.d11.into_opendrain(),
        // important since we light segments by opening for ground from 5V
        display_state: PinState::Low,
    };

    let ff_button = pins.d13.into_opendrain();

    let single_digits: RangeInclusive<u8> = 0..=9;
    let cycle = single_digits.clone().chain(single_digits.rev()).cycle();
    // back and forth, repeating the end values an extra time
    for next in cycle {
        display.set_digit(next).expect("handles all digits 0..=9");
        if ff_button.is_high() {
            arduino_hal::delay_ms(70);
        } else {
            arduino_hal::delay_ms(250);
        }
    }

    unreachable!("iterator cycles forever")
}
