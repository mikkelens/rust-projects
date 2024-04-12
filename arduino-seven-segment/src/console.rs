#![allow(unused)]

/*!
 * This is an example of how one can define a println!() macro which can be called anywhere to
 * write data to the console.
 *
 * Keep in mind that this will enter a critical section while printing so no interrupts can be
 * served in the meantime.
 *
 * src: https://github.com/Rahix/avr-hal/blob/main/examples/arduino-uno/src/bin/uno-println.rs
 */

use avr_device::interrupt;
use core::cell::RefCell;

pub type Console = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;

pub static CONSOLE: interrupt::Mutex<RefCell<Option<Console>>> =
    interrupt::Mutex::new(RefCell::new(None));

macro_rules! print {
    ($($t:tt)*) => {
        interrupt::free(
            |cs| {
                if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwrite!(console, $($t)*);
                }
            },
        )
    };
}
pub(crate) use print;

macro_rules! println {
    ($($t:tt)*) => {
        interrupt::free(
            |cs| {
                if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwriteln!(console, $($t)*);
                }
            },
        )
    };
}
pub(crate) use println;

pub(crate) fn put_console(console: Console) {
    interrupt::free(|cs| {
        *CONSOLE.borrow(cs).borrow_mut() = Some(console);
    })
}
