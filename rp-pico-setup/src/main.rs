#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(iter_repeat_n)]

use core::fmt::Debug;

use embassy_executor::Spawner;
use embassy_rp::pwm::Config;
use embassy_rp::{
    pwm::{Channel, Pwm},
    Peripheral,
};
use embassy_time::{Duration, Timer};

// necessary to be able to build: rtt for linking and required panic handler...
#[allow(unused_imports)] // ...even though we don't use anything from them
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::info!("Initializing...");
    let mut peripherals = embassy_rp::init(Default::default());

    let mut led = RGBLed {
        r: Pwm::new_output_a(
            unsafe { peripherals.PWM_CH1.clone_unchecked() },
            peripherals.PIN_18,
            Config::default(),
        ),
        g: Pwm::new_output_b(peripherals.PWM_CH1, peripherals.PIN_19, Config::default()),
        b: Pwm::new_output_a(peripherals.PWM_CH2, peripherals.PIN_20, Config::default()),
    };
    defmt::info!("Initialized.");

    led.cycle_counter().await;
    unreachable!("cyclic iter")
}

const PWM_MAX: u16 = u16::MAX;

struct RGBLed<'d, R: Channel, G: Channel, B: Channel> {
    r: Pwm<'d, R>,
    g: Pwm<'d, G>,
    b: Pwm<'d, B>,
}
impl<'d, R: Channel, G: Channel, B: Channel> RGBLed<'d, R, G, B> {
    fn set_counter_max(&mut self) {
        self.r.set_counter(PWM_MAX);
        self.g.set_counter(PWM_MAX);
        self.b.set_counter(PWM_MAX);
    }

    #[allow(unused)]
    async fn cycle_counter(&mut self) {
        let fade_up = 0..=PWM_MAX;
        let r_iter = fade_up
            .clone()
            .chain(core::iter::repeat_n(*fade_up.end(), fade_up.len() * 2))
            .chain(fade_up.clone().rev())
            .chain(core::iter::repeat_n(*fade_up.start(), fade_up.len()))
            .cycle();
        let g_iter = r_iter.clone().skip(fade_up.len());
        let b_iter = g_iter.clone().skip(fade_up.len());
        for (r_lvl, (g_lvl, b_lvl)) in r_iter.zip(g_iter.zip(b_iter)).step_by(200) {
            let mut slice_1 = Config::default();
            slice_1.compare_a = r_lvl;
            slice_1.compare_b = g_lvl;
            self.r.set_config(&slice_1);
            // pin for g is already set

            self.b.set_config(&{
                let mut c = Config::default();
                c.compare_a = b_lvl;
                c
            });
            defmt::info!(
                "r = {}, g = {}, b = {}",
                r_lvl as f32 / PWM_MAX as f32,
                g_lvl as f32 / PWM_MAX as f32,
                b_lvl as f32 / PWM_MAX as f32
            );

            // next step: use fixed point trigonometry to make it a (co)sine wave?

            Timer::after(Duration::from_millis(15)).await
        }
    }
}
