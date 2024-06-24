use embassy_rp::peripherals::{PWM_CH1, PWM_CH2, PWM_CH3};
use embassy_rp::pwm::{Channel, Config, Pwm};
use embassy_time::{Duration, Instant, Timer};
use fixed::prelude::*;
use fixed::types::extra::*;
use fixed::*;
use fixed_trigonometry::*;
use micromath::F32Ext;

/// Representing LED with the three component diodes as Pwm pins.
/// To use correctly, construct it with its component Pwm-s using unique PWM channels,
/// and use only A-channel pins (even numbered).
pub(crate) struct RGBLed<'d, R: Channel, G: Channel, B: Channel> {
    pub(crate) r: Pwm<'d, R>,
    pub(crate) g: Pwm<'d, G>,
    pub(crate) b: Pwm<'d, B>,
}

#[embassy_executor::task]
pub(crate) async fn loop_waves(mut led: RGBLed<'static, PWM_CH1, PWM_CH2, PWM_CH3>) -> ! {
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
        led.r.set_config(&config);

        let g = calculate_corrected(cycles + third_of_cycle);
        config.compare_a = g.to_bits();
        led.g.set_config(&config);

        let b = calculate_corrected(cycles + third_of_cycle * 2);
        config.compare_a = b.to_bits();
        led.b.set_config(&config);

        //        defmt::info!(
        //            "r={=f32}; g={=f32}; b={=f32}",
        //            r.to_num(),
        //            g.to_num(),
        //            b.to_num(),
        //        );

        Timer::after(Duration::from_millis(40)).await;
    }
}
