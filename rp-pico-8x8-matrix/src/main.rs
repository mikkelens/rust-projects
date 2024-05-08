#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::println;
use embassy_executor::Spawner;

// necessary to build:
#[allow(unused_imports)]
use {
    defmt_rtt as _,   // - rtt for linking
    embassy_rp::gpio, // - module includes stuff necessary for linking?
    panic_probe as _, // - no_std requires panic handler
};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::info!("Initializing...");
    let peripherals = embassy_rp::init(Default::default());
}
