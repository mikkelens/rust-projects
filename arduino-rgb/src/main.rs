#![feature(iter_repeat_n)]
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

use arduino_hal::port::mode::*;
use arduino_hal::port::*;
use arduino_hal::simple_pwm::*;
use avr_device::atmega328p::*;

mod panic;

struct ColorLed<R, G, B> {
    red: R,
    green: G,
    blue: B,
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);

    let curve = {
        const MAX: u8 = 255;
        let transition = 0..=MAX;
        let len = transition.clone().count();
        assert_eq!(
            MAX as usize + 1,
            len,
            "just making sure I understand range iters correctly"
        );

        let wait_len = len * 2;
        let red_curve = transition
            .clone()
            .chain(core::iter::repeat_n(MAX, wait_len))
            .chain(transition.rev())
            .chain(core::iter::repeat_n(0, wait_len))
            .cycle();
        let blue_curve = red_curve.clone().skip(len);
        let green_curve = blue_curve.clone().skip(len);
        red_curve.zip(blue_curve.zip(green_curve))
    };

    let mut led = ColorLed {
        red: pins.d5.into_output().into_pwm(&timer0),
        green: pins.d6.into_output().into_pwm(&timer0),
        blue: pins.d9.into_output().into_pwm(&timer1),
    };
    led.red.enable();
    led.green.enable();
    led.blue.enable();

    let input = pins.d2;

    for (r, (g, b)) in curve {
        led.red.set_duty(r);
        led.green.set_duty(g);
        led.blue.set_duty(b);
        arduino_hal::delay_ms(if input.is_high() { 1 } else { 16 });
    }
    unreachable!();
}
