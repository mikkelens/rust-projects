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

use arduino_hal::port::mode::{OpenDrain, Output};
use arduino_hal::port::{Pin, PinOps};
use core::ops::{BitXor, RangeInclusive};
use embedded_hal::digital::{OutputPin, PinState};

mod panic;

/// Attempt at representing a 4-digit 7-segment LED. It uses multiplexing.
/// https://softwareparticles.com/learn-how-a-4-digit-7-segment-led-display-works-and-how-to-control-it-using-an-arduino/
/// Different generics are needed because each pin is a different concrete type.
struct FourDigitSevenSegment<
    A: PinOps,
    B: PinOps,
    C: PinOps,
    D: PinOps,
    E: PinOps,
    F: PinOps,
    G: PinOps,
    DP: PinOps,
    D1: PinOps,
    D2: PinOps,
    D3: PinOps,
    D4: PinOps,
> {
    a: Pin<Output, A>,   // top bar
    b: Pin<Output, B>,   // upper right
    c: Pin<Output, C>,   // lower right
    d: Pin<Output, D>,   // bottom bar
    e: Pin<Output, E>,   // lower left
    f: Pin<Output, F>,   // upper left
    g: Pin<Output, G>,   // middle bar
    dp: Pin<Output, DP>, // dot in the bottom right corner
    // display_state: PinState,
    d1: Pin<Output, D1>,
    d2: Pin<Output, D2>,
    d3: Pin<Output, D3>,
    d4: Pin<Output, D4>,
}
impl<
        A: PinOps,
        B: PinOps,
        C: PinOps,
        D: PinOps,
        E: PinOps,
        F: PinOps,
        G: PinOps,
        DP: PinOps,
        D1: PinOps,
        D2: PinOps,
        D3: PinOps,
        D4: PinOps,
    > FourDigitSevenSegment<A, B, C, D, E, F, G, DP, D1, D2, D3, D4>
{
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let display = FourDigitSevenSegment {
        a: pins.d6.into_output(), // for pin source, see link in type documentation
        b: pins.d7.into_output(),
        c: pins.d8.into_output(),
        d: pins.d9.into_output(),
        e: pins.d10.into_output(),
        f: pins.d11.into_output(),
        g: pins.d12.into_output(),
        dp: pins.d13.into_output(),
        // display_state: PinState::Low,
        d1: pins.d2.into_output(),
        d2: pins.d3.into_output(),
        d3: pins.d4.into_output(),
        d4: pins.d5.into_output(),
    };

    loop {}
}

// fn represent_digit(&mut self, num: u8) -> Result<(), DigitError> {
//     let on = self.display_state;
//     let off = !on;
//     match num {
//         0 => {
//             self.a.set_state(on).unwrap(); // set_state error type is `Infallible`
//             self.b.set_state(on).unwrap();
//             self.c.set_state(on).unwrap();
//             self.d.set_state(on).unwrap();
//             self.e.set_state(on).unwrap();
//             self.f.set_state(on).unwrap();
//             self.g.set_state(off).unwrap();
//         }
//         1 => {
//             self.a.set_state(off).unwrap(); // set_state error type is `Infallible`
//             self.b.set_state(on).unwrap();
//             self.c.set_state(on).unwrap();
//             self.d.set_state(off).unwrap();
//             self.e.set_state(off).unwrap();
//             self.f.set_state(off).unwrap();
//             self.g.set_state(off).unwrap();
//         }
//         2 => {
//             self.a.set_state(on).unwrap(); // set_state error type is `Infallible`
//             self.b.set_state(on).unwrap();
//             self.c.set_state(off).unwrap();
//             self.d.set_state(on).unwrap();
//             self.e.set_state(on).unwrap();
//             self.f.set_state(off).unwrap();
//             self.g.set_state(on).unwrap();
//         }
//         3 => {
//             self.a.set_state(on).unwrap(); // set_state error type is `Infallible`
//             self.b.set_state(on).unwrap();
//             self.c.set_state(on).unwrap();
//             self.d.set_state(on).unwrap();
//             self.e.set_state(off).unwrap();
//             self.f.set_state(off).unwrap();
//             self.g.set_state(on).unwrap();
//         }
//         4 => {
//             self.a.set_state(off).unwrap(); // set_state error type is `Infallible`
//             self.b.set_state(on).unwrap();
//             self.c.set_state(on).unwrap();
//             self.d.set_state(off).unwrap();
//             self.e.set_state(off).unwrap();
//             self.f.set_state(on).unwrap();
//             self.g.set_state(on).unwrap();
//         }
//         5 => {
//             self.a.set_state(on).unwrap(); // set_state error type is `Infallible`
//             self.b.set_state(off).unwrap();
//             self.c.set_state(on).unwrap();
//             self.d.set_state(on).unwrap();
//             self.e.set_state(off).unwrap();
//             self.f.set_state(on).unwrap();
//             self.g.set_state(on).unwrap();
//         }
//         6 => {
//             self.a.set_state(on).unwrap(); // set_state error type is `Infallible`
//             self.b.set_state(off).unwrap();
//             self.c.set_state(on).unwrap();
//             self.d.set_state(on).unwrap();
//             self.e.set_state(on).unwrap();
//             self.f.set_state(on).unwrap();
//             self.g.set_state(on).unwrap();
//         }
//         7 => {
//             self.a.set_state(on).unwrap(); // set_state error type is `Infallible`
//             self.b.set_state(on).unwrap();
//             self.c.set_state(on).unwrap();
//             self.d.set_state(off).unwrap();
//             self.e.set_state(off).unwrap();
//             self.f.set_state(off).unwrap();
//             self.g.set_state(off).unwrap();
//         }
//         8 => {
//             self.a.set_state(on).unwrap(); // set_state error type is `Infallible`
//             self.b.set_state(on).unwrap();
//             self.c.set_state(on).unwrap();
//             self.d.set_state(on).unwrap();
//             self.e.set_state(on).unwrap();
//             self.f.set_state(on).unwrap();
//             self.g.set_state(on).unwrap();
//         }
//         9 => {
//             self.a.set_state(on).unwrap(); // set_state error type is `Infallible`
//             self.b.set_state(on).unwrap();
//             self.c.set_state(on).unwrap();
//             self.d.set_state(on).unwrap();
//             self.e.set_state(off).unwrap();
//             self.f.set_state(on).unwrap();
//             self.g.set_state(on).unwrap();
//         }
//         10.. => return Err(DigitError::ValueTooBig),
//     }
//     Ok(())
// }
// fn dot_state(&mut self, state: bool) {
//     let desired = if state {
//         self.display_state
//     } else {
//         !self.display_state
//     };
//     self.dp.set_state(desired).unwrap()
// }
