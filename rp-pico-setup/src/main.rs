#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(iter_repeat_n)]

use embassy_executor::Spawner;
use embassy_rp::{
	pwm,
	pwm::{Channel, Pwm},
	Peripheral
};
use embassy_time::{Duration, Timer};
// necessary to be able to build: rtt for linking and required panic handler...
#[allow(unused_imports)] // ...even though we don't use anything from them
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
	defmt::info!("Initializing...");
	let mut peripherals = embassy_rp::init(Default::default());

	let mut config: pwm::Config = Default::default();
	config.top = u16::MAX / 2;
	config.compare_a = 8;

	let mut led = RGBLed {
		r: Pwm::new_output_a(
			unsafe { peripherals.PWM_CH1.clone_unchecked() },
			peripherals.PIN_18,
			config.clone()
		),
		g: Pwm::new_output_b(peripherals.PWM_CH1, peripherals.PIN_19, config.clone()),
		b: Pwm::new_output_a(peripherals.PWM_CH2, peripherals.PIN_20, config.clone())
	};
	defmt::info!("Initialized.");

	led.cycle_counter().await;
	unreachable!("cyclic iter")
}

const PWM_MAX: u16 = u16::MAX / 2;

struct RGBLed<'d, R: Channel, G: Channel, B: Channel> {
	r: Pwm<'d, R>,
	g: Pwm<'d, G>,
	b: Pwm<'d, B>
}
impl<'d, R: Channel, G: Channel, B: Channel> RGBLed<'d, R, G, B> {
	fn set_counter_max(&mut self) {
		self.r.set_counter(PWM_MAX);
		self.g.set_counter(PWM_MAX);
		self.b.set_counter(PWM_MAX);
	}

	#[allow(unused)]
	async fn cycle_counter(&mut self) {
		let up_fade = 0..=PWM_MAX;
		let r_iter = up_fade
			.clone()
			.chain(core::iter::repeat_n(*up_fade.end(), up_fade.len()))
			.chain(up_fade.clone().rev())
			.chain(core::iter::repeat_n(*up_fade.start(), up_fade.len()));
		let g_iter = r_iter.clone().skip(up_fade.len());
		let b_iter = g_iter.clone().skip(up_fade.len());
		for (r_lvl, (g_lvl, b_lvl)) in r_iter.zip(g_iter.zip(b_iter)).cycle().step_by(50) {
			todo!("figure out how to change config from within here properly");
			// let r_config = self.r.config;
			// self.r.set_config(r_lvl);
			defmt::info!("r_count = {}", r_lvl);
			// let g_config = self.g.config;
			// self.g.set_config(g_lvl);
			defmt::info!("g_count = {}", g_lvl);
			// let b_config = self.b.config;
			// self.b.set_config(b_lvl);
			defmt::info!("b_count = {}", b_lvl);
			Timer::after(Duration::from_millis(5)).await
		}
	}
}
