#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(iter_map_windows)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output, Pin};
use embedded_alloc::Heap;

/// necessary to be able to build: rtt for linking and required panic handler,
/// even though we don't use anything from them
#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};

#[allow(unused_imports)]
use crate::{rgb_led::*, seven_segment::*, shift_register::*};

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum ActivationState {
    On,
    Off,
}
impl From<bool> for ActivationState {
    fn from(value: bool) -> Self {
        match value {
            true => Self::On,
            false => Self::Off,
        }
    }
}
//impl core::ops::Not for ActivationState {
//    type Output = Self;
//    fn not(self) -> Self::Output {
//        match self {
//            Self::On => Self::Off,
//            Self::Off => Self::On,
//        }
//    }
//}

pub(crate) trait CommonPolarity {
    const ON: Level;
    const OFF: Level;
}

#[allow(unused)]
mod rgb_led;
#[allow(unused)]
mod seven_segment;
#[allow(unused)]
mod shift_register;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    defmt::info!("Initializing...");

    let peripherals = embassy_rp::init(Default::default());

    #[allow(unused_mut)]
    let mut shift_register = ShiftRegisterSIPO {
        SRCLK: Output::new(peripherals.PIN_2.degrade(), Level::Low),
        RCLK: Output::new(peripherals.PIN_3.degrade(), Level::Low),
        SER: Output::new(peripherals.PIN_4.degrade(), Level::Low),
        // hardwired pins:
        OE: None,
        SRCLR: None,
    };

    spawner
        .spawn(cycle_numbers_cathode(shift_register))
        .unwrap();

    // todo: implement some rudimentary wire(less?) communications (e.g. remote start, value editing/reset?)

    //    let mut config = Config::default();
    //    config.phase_correct = true;
    //    //    config.divider = 10u16.to_fixed();
    //    config.compare_a = 0x0fff;
    //    config.compare_b = 0xffff;
    //    let potentiometer = Pwm::new_input(
    //        peripherals.PWM_CH7,
    //        peripherals.PIN_15,
    //        InputMode::Level,
    //        config,
    //    );
    //    loop {
    //        defmt::info!("POTENTIOMETER: {}", potentiometer.counter());
    //        potentiometer.set_counter(0);
    //        Timer::after(Duration::from_millis(100)).await;
    //    }
}
