#![feature(associated_type_defaults)]
#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]
#![no_std]
#![no_main]

use bitvec::prelude::*;
use core::ops::Not;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output, Pin};
use embassy_time::Timer;
#[allow(unused_imports)] // necessary to build
use {defmt_rtt as _, panic_probe as _};

#[allow(unused_imports)] // convenience
use crate::shift_register::*;

mod shift_register;

//#[allow(unused_variables)]
//#[embassy_executor::main]
//async fn main(spawner: Spawner) {
//    let peripherals = embassy_rp::init(Default::default());
//    let register = ShiftRegisterSIPO {
//        SER: Output::new(peripherals.PIN_18.degrade(), Level::Low),
//        SRCLK: Output::new(peripherals.PIN_16.degrade(), Level::Low),
//        RCLK: None,
//        SRCLR: None,
//        OE: None,
//    };
//    spawner.spawn(run_endlessly(register)).unwrap();
//}
//
//#[embassy_executor::task]
//async fn run_endlessly(mut register: ShiftRegisterSIPO<'static, 16>) {
//    loop {
//        register.set_next_bit(Level::High);
//        Timer::after_millis(300).await;
//        register.set_next_bit(Level::Low);
//        Timer::after_millis(300).await;
//    }
//}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    let mut register = ShiftRegister::<8> {
        data: SerialOutput {
            SER: Output::new(peripherals.PIN_16.degrade(), Level::Low),
            SRCLK: Output::new(peripherals.PIN_17.degrade(), Level::Low),
        },
        RCLK: Output::new(peripherals.PIN_18.degrade(), Level::Low),
    };
    defmt::info!("cycling low/high each 100 ms!");
    let delay = 200;
    let range = 0..=u8::MAX;
    //    let c = a.chain(b).repeat();
    for num in range.clone().chain(range.rev()).cycle() {
        let a: BitArray<_> = num.into_bitarray();
        let b = a
            .iter()
            .by_vals()
            .map(bool::not)
            .map(Level::from)
            .collect::<heapless::Vec<Level, 8>>()
            .into_array()
            .unwrap();
        register.write_full(b);
        Timer::after_millis(delay).await;
    }
}
