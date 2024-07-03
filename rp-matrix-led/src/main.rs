#![feature(associated_type_defaults)]
#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]
#![no_std]
#![no_main]

mod matrix_led;
mod shift_register;

#[allow(unused_imports)] // necessary to build
use {defmt_rtt as _, panic_probe as _};

#[allow(unused_imports)] // convenience
use crate::{matrix_led::*, shift_register::*};

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output, Pin};
use embassy_time::{Duration, Timer};
use heapless::Vec;

#[allow(unused_variables)]
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    let display = RedDisplay {
        sipo: ShiftRegisterSIPO {
            SER: Output::new(peripherals.PIN_18.degrade(), Level::Low),
            SRCLK: Output::new(peripherals.PIN_16.degrade(), Level::Low),
            RCLK: Output::new(peripherals.PIN_17.degrade(), Level::Low),
            SRCLR: None,
            OE: None,
        },
        image: [[RedLEDState::Off; 8]; 8],
    };
    spawner.spawn(run_endlessly(display)).unwrap();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum RedLEDState {
    Off,
    Red,
}
impl From<Level> for RedLEDState {
    fn from(value: Level) -> Self {
        match value {
            Level::Low => Self::Off,
            Level::High => Self::Red,
        }
    }
}
impl Into<Level> for RedLEDState {
    fn into(self) -> Level {
        match self {
            RedLEDState::Off => Level::Low,
            RedLEDState::Red => Level::High,
        }
    }
}
impl LEDState for RedLEDState {
    fn default_on() -> Self {
        Self::Red
    }
}

struct RedDisplay<'d, const PINS: usize> {
    sipo: ShiftRegisterSIPO<'d, PINS>,
    image: [[RedLEDState; 8]; 8],
}

#[embassy_executor::task]
async fn run_endlessly(mut display: RedDisplay<'static, 16>) {
    loop {
        for y in 0..8 {
            let mut off_y = [Level::Low; 8];
            off_y[y] = Level::High;
            for x in 0..8 {
                let mut off_x = [Level::Low; 8];
                off_x[x] = Level::High;
                display.sipo.set_full(
                    &[off_y, off_x]
                        .into_iter()
                        .flatten()
                        .collect::<Vec<_, 16>>()
                        .into_array()
                        .unwrap(),
                );
                Timer::after(Duration::from_millis(200)).await;
            }
        }
    }
}
