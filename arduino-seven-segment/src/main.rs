#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

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
use embedded_hal::digital::{OutputPin, PinState};

mod countdown;
mod cycle;

pub mod console;
pub mod millis;
pub mod panic;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();

    // cycle::run(pins)
    countdown::run(dp)
}

/// Representing a physical seven segment LED display.
/// For diagram of what the labels mean, see link:
/// https://components101.com/sites/default/files/component_pin/7-segment-display-pin-diagr_0.png
/// Different generics are needed because each pin is a different concrete type.
pub struct SevenSegment<
    A: PinOps,
    B: PinOps,
    C: PinOps,
    D: PinOps,
    E: PinOps,
    F: PinOps,
    G: PinOps,
    DP: PinOps,
> {
    pub a: Pin<OpenDrain, A>,   // top bar
    pub b: Pin<OpenDrain, B>,   // upper right
    pub c: Pin<OpenDrain, C>,   // lower right
    pub d: Pin<OpenDrain, D>,   // bottom bar
    pub e: Pin<OpenDrain, E>,   // lower left
    pub f: Pin<OpenDrain, F>,   // upper left
    pub g: Pin<OpenDrain, G>,   // middle bar
    pub dp: Pin<OpenDrain, DP>, // dot in the bottom right corner
    pub display_state: PinState,
}
#[derive(Debug)]
pub enum DigitError {
    ValueTooBig,
}
impl<A: PinOps, B: PinOps, C: PinOps, D: PinOps, E: PinOps, F: PinOps, G: PinOps, DP: PinOps>
    SevenSegment<A, B, C, D, E, F, G, DP>
{
    /// Sets digit to a specific value.
    /// Does not touch the decimal point (`DP`).
    pub fn set_digit(&mut self, num: u8) -> Result<(), DigitError> {
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
    pub fn show_decimal(&mut self, state: bool) {
        let desired = if state {
            self.display_state
        } else {
            !self.display_state
        };
        self.dp.set_state(desired).unwrap()
    }
}
