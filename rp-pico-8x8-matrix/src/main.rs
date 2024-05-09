#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_rp::gpio::{AnyPin, Level, Output, Pin};
use embassy_time::Duration;
// necessary to build:
#[allow(unused_imports)]
use {
	defmt_rtt as _,   // - rtt for linking
	embassy_rp::gpio, // - module includes stuff necessary for linking?
	panic_probe as _  // - no_std requires panic handler
};

/// direction is assumed
struct Display<'a> {
	v: [Output<'a, AnyPin>; 8],
	r: [Output<'a, AnyPin>; 8],
	g: [Output<'a, AnyPin>; 8]
}
impl Display {
	fn animate_blocking_r(&mut self, level: Level) {
		for pin in self.r.iter_mut() {
			pin.set_level(level);
			embassy_time::block_for(Duration::from_millis(350));
		}
	}
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
	defmt::info!("Initializing...");
	let peripherals = embassy_rp::init(Default::default());
	let mut display = Display {
		v: [
			Output::new(peripherals.PIN_0.degrade(), Level::Low),
			Output::new(peripherals.PIN_1.degrade(), Level::Low),
			Output::new(peripherals.PIN_2.degrade(), Level::Low),
			Output::new(peripherals.PIN_3.degrade(), Level::Low),
			Output::new(peripherals.PIN_4.degrade(), Level::Low),
			Output::new(peripherals.PIN_5.degrade(), Level::Low),
			Output::new(peripherals.PIN_6.degrade(), Level::Low),
			Output::new(peripherals.PIN_7.degrade(), Level::Low)
		],
		r: [
			Output::new(peripherals.PIN_8.degrade(), Level::Low),
			Output::new(peripherals.PIN_9.degrade(), Level::Low),
			Output::new(peripherals.PIN_10.degrade(), Level::Low),
			Output::new(peripherals.PIN_11.degrade(), Level::Low),
			Output::new(peripherals.PIN_12.degrade(), Level::Low),
			Output::new(peripherals.PIN_13.degrade(), Level::Low),
			Output::new(peripherals.PIN_14.degrade(), Level::Low),
			Output::new(peripherals.PIN_15.degrade(), Level::Low)
		],
		g: [
			Output::new(peripherals.PIN_8.degrade(), Level::Low),
			Output::new(peripherals.PIN_9.degrade(), Level::Low),
			Output::new(peripherals.PIN_10.degrade(), Level::Low),
			Output::new(peripherals.PIN_11.degrade(), Level::Low),
			Output::new(peripherals.PIN_12.degrade(), Level::Low),
			Output::new(peripherals.PIN_13.degrade(), Level::Low),
			Output::new(peripherals.PIN_14.degrade(), Level::Low),
			Output::new(peripherals.PIN_15.degrade(), Level::Low)
		]
	};
	loop {
		display.animate_blocking_r(Level::High);
		display.animate_blocking_r(Level::Low);
	}
}
