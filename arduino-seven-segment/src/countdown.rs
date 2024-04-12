use avr_device::atmega328p::Peripherals;
use embedded_hal::digital::PinState;

#[allow(unused_imports)] // used by macro
use {avr_device::interrupt, console::CONSOLE};

use crate::{console, millis, SevenSegment};

struct ChargedTimer {
    state: ChargeState,
    unused_time: u32,
}
enum ChargeState {
    Pressed(u32), // energy
    Unpressed(u32),
}

#[allow(unused)]
pub fn run(dp: Peripherals) -> ! {
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    console::put_console(serial);
    console::println!("running countdown program...");

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

    let press_button = pins.d13.into_opendrain();

    millis::millis_init(dp.TC0);
    unsafe { interrupt::enable() }; // Enable interrupts globally

    let mut timer = ChargedTimer {
        state: ChargeState::Unpressed(millis::millis()),
        unused_time: 0,
    };

    loop {
        let current = millis::millis();

        let increment_size = 225;
        let max_energy = 10 * increment_size;

        // update state
        let energy = match timer.state {
            ChargeState::Pressed(start) => {
                if press_button.is_low() {
                    timer.state = ChargeState::Unpressed(current);
                    timer.unused_time =
                        (timer.unused_time + current - start).min(max_energy);
                    continue;
                }
                current - start + timer.unused_time // goes up over time from some value (often zero)
            }
            ChargeState::Unpressed(start) => {
                if press_button.is_high() {
                    timer.state = ChargeState::Pressed(current);
                    timer.unused_time = timer.unused_time.saturating_sub(current - start);
                    continue;
                }
                (start + timer.unused_time).saturating_sub(current) // goes towards zero
            }
        }.min(max_energy);
        let increments = energy / increment_size;
        let digit = increments.min(9) as u8;

        // console::println!(
        //     "current={}, time={}, increments={}, digit={}",
        //     current,
        //     energy,
        //     increments,
        //     digit
        // );

        display
            .set_digit(digit)
            .expect("min keeps unsigned integer 0..=9");
        display.show_decimal(energy == 0 || energy == max_energy)

        // arduino_hal::delay_ms(25);
    }
}
