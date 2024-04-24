#![feature(abi_avr_interrupt)] // needed for millis() functionality
#![no_std]
#![no_main]

/*
 * For examples (and inspiration), head to https://github.com/Rahix/avr-hal/tree/main/examples
 */

#[allow(unused_imports)]
use {avr_device::interrupt, color_led::*, display::*, print::*};

mod characters;
mod color_led;
mod display;
mod millis;
mod panic;
mod print;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    put_console(serial);

    let mut display = FourDigitDisplay {
        segments: SevenSegmentPlusDot {
            // a-g
            a: pins.d9.into_output(),
            b: pins.d2.into_output(),
            c: pins.d3.into_output(),
            d: pins.d5.into_output(),
            e: pins.d6.into_output(),
            f: pins.d8.into_output(),
            g: pins.d7.into_output(),
            // dot point
            dp: pins.d4.into_output(),
            // led state type
            common: Common::Anode,
        }
        .with_all_off(),
        // D1-4
        digit_pins: FourDigitPins {
            d1: pins.d13.into_output(),
            d2: pins.d12.into_output(),
            d3: pins.d11.into_output(),
            d4: pins.d10.into_output(),
            common: Common::Cathode,
        }
        .with_all_off(),
        num: 0,
        digit_rotation: ROTATION.into_iter().cycle(),
    };

    let button = pins.a0.into_pull_up_input();
    let mut pause = false;
    let mut released = true;

    let new_num_delay = 50;

    let numbers = 0..=9000u16;
    let cyclic_iter = numbers.clone().chain(numbers.rev()).cycle();
    for num in cyclic_iter {
        println!("{}", num);
        for _ in 0..new_num_delay {
            loop {
                let button_on = button.is_low();
                display.keep_active_for_ms();

                if button_on {
                    if released {
                        pause = !pause;
                        released = false;
                    }
                } else {
                    released = true;
                }

                if !pause {
                    break;
                }
            }
        }
        display.num = num;
        // println!("{}", num);
    }

    unreachable!("Cyclic iterator never finishes.")
}

#[allow(unused)]
const U8_SINGLE_BIT_ITER: [u8; 8] = [
    0b00000001u8, // A
    0b00000010u8, // B
    0b00000100u8, // C
    0b00001000u8, // D
    0b00010000u8, // E
    0b00100000u8, // F
    0b01000000u8, // G
    0b10000000u8, // should not be visible
];
