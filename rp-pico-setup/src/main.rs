#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(iter_repeat_n)]

use embassy_executor::Spawner;
use embassy_rp::pwm::Config;
use embassy_rp::pwm::{Channel, Pwm};
use embassy_time::{Duration, Instant, Timer};
use embedded_alloc::Heap;
use fixed::prelude::{FromFixed, ToFixed};
use fixed::types::extra::*;
use fixed::*;
use fixed_trigonometry::*;
use micromath::*;

// necessary to be able to build: rtt for linking and required panic handler...
#[allow(unused_imports)] // ...even though we don't use anything from them
use {defmt_rtt as _, panic_probe as _};

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::info!("Initializing...");
    let peripherals = embassy_rp::init(Default::default());

    let mut led = RGBLed {
        r: Pwm::new_output_a(peripherals.PWM_CH1, peripherals.PIN_18, Config::default()),
        g: Pwm::new_output_a(peripherals.PWM_CH2, peripherals.PIN_20, Config::default()),
        b: Pwm::new_output_a(peripherals.PWM_CH3, peripherals.PIN_22, Config::default()),
    };
    defmt::info!("Initialized.");

    //    led.cycle_linear_hsv().await;
    led.loop_waves().await;
    unreachable!("cyclic iter")
}

const PWM_MAX: u16 = u16::MAX;

/// Representing LED with the three component diodes as Pwm pins.
/// To use correctly, construct it with its component Pwm-s using unique PWM channels,
/// and use only A-channel pins (even numbered).
struct RGBLed<'d, R: Channel, G: Channel, B: Channel> {
    r: Pwm<'d, R>,
    g: Pwm<'d, G>,
    b: Pwm<'d, B>,
}
impl<'d, R: Channel, G: Channel, B: Channel> RGBLed<'d, R, G, B> {
    #[allow(unused)]
    fn set_counter_max(&mut self) {
        self.r.set_counter(PWM_MAX);
        self.g.set_counter(PWM_MAX);
        self.b.set_counter(PWM_MAX);
    }

    #[allow(unused)]
    async fn cycle_linear_hsv(&mut self) {
        let fade_up = 0..=PWM_MAX;
        let r_iter = fade_up
            .clone()
            .chain(core::iter::repeat_n(*fade_up.end(), fade_up.len() * 2))
            .chain(fade_up.clone().rev())
            .chain(core::iter::repeat_n(*fade_up.start(), fade_up.len()))
            .cycle();
        let g_iter = r_iter.clone().skip(fade_up.len());
        let b_iter = g_iter.clone().skip(fade_up.len());

        let mut config = Config::default();
        for (r_lvl, (g_lvl, b_lvl)) in r_iter.zip(g_iter.zip(b_iter)).step_by(20000) {
            config.compare_a = r_lvl;
            self.r.set_config(&config);
            config.compare_a = g_lvl;
            self.g.set_config(&config);
            config.compare_a = b_lvl;
            self.b.set_config(&config);

            defmt::info!(
                "\nr={=f32};\ng={=f32};\nb={=f32}",
                r_lvl as f32 / PWM_MAX as f32,
                g_lvl as f32 / PWM_MAX as f32,
                b_lvl as f32 / PWM_MAX as f32
            );

            Timer::after(Duration::from_millis(500)).await
        }
    }

    #[allow(unused)]
    async fn loop_waves(&mut self) {
        #[inline]
        fn calculate_corrected(point: FixedU64<U32>) -> FixedU16<U16> {
            let phase_time = wrap_phase(FixedI32::<U16>::wrapping_from_fixed(point));
            let signed_cos = cos(phase_time);
            let unsigned_cos: FixedU16<U16> =
                ((signed_cos + FixedI32::from_num(1)) / 2).saturating_to_fixed();
            FixedU16::<U16>::from_num(unsigned_cos.to_num::<f32>().powf(2.2))
        }

        defmt::info!("Prepared math, stepping...");
        let mut config = Config::default();
        loop {
            let micros = FixedU64::<U32>::wrapping_from_num(Instant::now().as_micros());
            let seconds = micros / 1_000_000;
            let cycles_per_second = FixedU64::<U32>::from_num(5);
            let cycles = (seconds / cycles_per_second) * FixedU64::TAU;
            let third_of_cycle = FixedU64::TAU / 3;

            let r = calculate_corrected(cycles);
            config.compare_a = r.to_bits();
            self.r.set_config(&config);

            let g = calculate_corrected(cycles + third_of_cycle);
            config.compare_a = g.to_bits();
            self.g.set_config(&config);

            let b = calculate_corrected(cycles + third_of_cycle * 2);
            config.compare_a = b.to_bits();
            self.b.set_config(&config);

            defmt::info!(
                "r={=f32}; g={=f32}; b={=f32}",
                r.to_num(),
                g.to_num(),
                b.to_num(),
            );

            Timer::after(Duration::from_millis(25)).await;
        }
    }
}
