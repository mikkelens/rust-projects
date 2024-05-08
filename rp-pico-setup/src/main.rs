#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_rp::gpio::{AnyPin, Level, Output, Pin};
use embassy_time::{Duration, Timer};
use itertools::Itertools;

// necessary to be able to build: rtt for linking and required panic handler...
#[allow(unused_imports)] // ...even though we don't use anything from them
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::info!("Initializing...");
    let peripherals = embassy_rp::init(Default::default());

    let mut display = MatrixDisplay {
        // first two pins (0-1) are for debugging (start at PIN-2), and pins 23-25 do not exist (skip) on pico-W
        v: [
            Output::new(peripherals.PIN_28.degrade(), Level::High),
            Output::new(peripherals.PIN_27.degrade(), Level::High),
            Output::new(peripherals.PIN_26.degrade(), Level::High),
            Output::new(peripherals.PIN_22.degrade(), Level::High),
            Output::new(peripherals.PIN_5.degrade(), Level::High), // wraps around
            Output::new(peripherals.PIN_4.degrade(), Level::High),
            Output::new(peripherals.PIN_3.degrade(), Level::High),
            Output::new(peripherals.PIN_2.degrade(), Level::High),
        ],
        g: [
            Output::new(peripherals.PIN_6.degrade(), Level::Low),
            Output::new(peripherals.PIN_7.degrade(), Level::Low),
            Output::new(peripherals.PIN_8.degrade(), Level::Low),
            Output::new(peripherals.PIN_9.degrade(), Level::Low),
            Output::new(peripherals.PIN_10.degrade(), Level::Low),
            Output::new(peripherals.PIN_11.degrade(), Level::Low),
            Output::new(peripherals.PIN_12.degrade(), Level::Low),
            Output::new(peripherals.PIN_13.degrade(), Level::Low),
        ],

        r: [
            Output::new(peripherals.PIN_21.degrade(), Level::Low),
            Output::new(peripherals.PIN_20.degrade(), Level::Low),
            Output::new(peripherals.PIN_19.degrade(), Level::Low),
            Output::new(peripherals.PIN_18.degrade(), Level::Low),
            Output::new(peripherals.PIN_17.degrade(), Level::Low),
            Output::new(peripherals.PIN_16.degrade(), Level::Low),
            Output::new(peripherals.PIN_15.degrade(), Level::Low),
            Output::new(peripherals.PIN_14.degrade(), Level::Low),
        ],
    };

    defmt::info!("Initialized.");

    display.try_different_pins().await;
    // loop {}
    unreachable!("cyclic iter")
}

struct MatrixDisplay<'d> {
    // common anode/VCC
    v: [Output<'d, AnyPin>; 8],
    // green
    g: [Output<'d, AnyPin>; 8],
    // red
    r: [Output<'d, AnyPin>; 8],
}

impl<'d> MatrixDisplay<'d> {
    async fn try_different_pins(&mut self) {
        let fmt = |lvl| if lvl == Level::High { "HIGH" } else { "LOW" };
        let wait = Duration::from_millis(50);
        let pin_id_range = 0..8;
        // all (2^2) combinations of turned on/off
        for (r_level, g_level) in [
            (Level::Low, Level::High),
            (Level::High, Level::Low),
            (Level::Low, Level::Low),
            (Level::High, Level::High),
        ]
        .into_iter()
        .cycle()
        {
            defmt::info!(
                "Trying combinations of r={} and g={}",
                fmt(r_level),
                fmt(g_level)
            );
            // all (8^2) combinations of id-s
            for (r, g) in pin_id_range.clone().tuple_combinations() {
                let prev_r = self.r[r].get_output_level();
                let prev_g = self.g[g].get_output_level();
                self.r[r].set_level(r_level);
                self.g[g].set_level(g_level);
                Timer::after(wait).await;
                self.r[r].set_level(prev_r);
                self.g[g].set_level(prev_g);
            }
            Timer::after(wait * 3).await;
        }
    }
}
