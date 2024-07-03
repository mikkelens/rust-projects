use either::Either;
use embassy_rp::gpio::Level;
use embassy_time::{Duration, Ticker, Timer};
use heapless::Vec;
use micromath::F32Ext;

use crate::seven_segment::types::*;
use crate::shift_register::ShiftRegisterSIPO;
use crate::*;

mod types {
    use embassy_time::Duration;
    use ActivationState::*;

    use crate::*;

    #[derive(Copy, Clone)]
    pub(crate) struct Segments<const AMOUNT: usize>(pub(crate) [ActivationState; AMOUNT]);
    pub(crate) struct SegmentsWithDot<const S: usize>(
        pub(crate) Segments<S>,
        pub(crate) ActivationState,
    );

    /// Common/controlling pins light up their segment when HIGH
    pub(crate) struct CommonAnode;
    impl CommonPolarity for CommonAnode {
        const ON: Level = Level::High;
        const OFF: Level = Level::Low;
    }
    /// Common/controlling pins light up their segment when LOW
    pub(crate) struct CommonCathode;
    impl CommonPolarity for CommonCathode {
        const ON: Level = Level::Low;
        const OFF: Level = Level::High;
    }

    #[derive(Debug)]
    pub(crate) enum DigitSegmentError {
        TooBig(u8),
    }
    impl Segments<7> {
        pub(crate) const fn digit_to_segments(digit: u8) -> Result<Segments<7>, DigitSegmentError> {
            match digit {
                0..=9 => Ok(Segments(match digit {
                    0 => [On, On, On, On, On, On, Off],
                    1 => [Off, On, On, Off, Off, Off, Off],
                    2 => [On, On, Off, On, On, Off, On],
                    3 => [On, On, On, On, Off, Off, On],
                    4 => [Off, On, On, Off, Off, On, On],
                    5 => [On, Off, On, On, Off, On, On],
                    6 => [On, Off, On, On, On, On, On],
                    7 => [On, On, On, Off, Off, Off, Off],
                    8 => [On, On, On, On, On, On, On],
                    9 => [On, On, On, Off, Off, On, On],
                    _ => unreachable!(),
                })),
                d => Err(DigitSegmentError::TooBig(d)),
            }
        }
        pub(crate) const NONE: Self = Self([Off; 7]);
        pub(crate) const ONLY_A: Self = Self([On, Off, Off, Off, Off, Off, Off]);
        pub(crate) const ONLY_B: Self = Self([Off, On, Off, Off, Off, Off, Off]);
        pub(crate) const ONLY_C: Self = Self([Off, Off, On, Off, Off, Off, Off]);
        pub(crate) const ONLY_D: Self = Self([Off, Off, Off, On, Off, Off, Off]);
        pub(crate) const ONLY_E: Self = Self([Off, Off, Off, Off, On, Off, Off]);
        pub(crate) const ONLY_F: Self = Self([Off, Off, Off, Off, Off, On, Off]);
        pub(crate) const ONLY_G: Self = Self([Off, Off, Off, Off, Off, Off, On]);
    }
    impl SegmentsWithDot<7> {
        pub(crate) const ONLY_DP: Self = Self(Segments::NONE, On);
    }

    pub(crate) trait SegmentDisplayWithDot<const S: usize, P: CommonPolarity> {
        fn clear_display(&mut self);
        fn display_segments(&mut self, segments: &SegmentsWithDot<S>);
    }

    pub(crate) trait SevenSegmentedDisplayWithDot<P: CommonPolarity>:
        SegmentDisplayWithDot<7, P>
    {
        async fn cycle_numbers(&mut self);
        async fn spin_animate<const FRAMES: usize>(
            &mut self,
            pattern: &[ActivationState; 6],
            counter_clockwise: bool,
            dot: ActivationState,
            delay_fn: impl Fn(f32) -> Duration,
            wait_time: Duration,
        );
    }
}

impl<'d, const S: usize, P: CommonPolarity> SegmentDisplayWithDot<S, P>
    for ShiftRegisterSIPO<'d, { S + 1 }>
{
    fn clear_display(&mut self) {
        self.write_full(&[P::OFF; S + 1]);
    }
    fn display_segments(&mut self, segments: &SegmentsWithDot<S>) {
        use ActivationState::*;
        self.write_full(
            &(segments
                .0
                 .0
                .iter()
                .copied()
                .chain(core::iter::once(segments.1))
                .map(|desired| match desired {
                    On => P::ON,
                    Off => P::OFF,
                })
                .collect::<Vec<_, { S + 1 }>>()
                .into_array()
                .unwrap()),
        );
    }
}

impl<'d, P: CommonPolarity> SevenSegmentedDisplayWithDot<P> for ShiftRegisterSIPO<'d, 8>
where
    Self: SegmentDisplayWithDot<7, P>,
{
    /// Animation where numbers ascend and descend on loop.
    /// Each end of change (reaching 0 and 9) has a related animation.
    async fn cycle_numbers(&mut self) {
        use ActivationState::*;
        // defmt::info!("Starting number cycle...");

        let a_pattern = [On, Off, Off, On, Off, Off]; // 1/3 segments, split
        let b_pattern = [On, On, Off, Off, Off, Off]; // 1/3 segments
        let c_pattern = [On, Off, Off, Off, Off, Off]; // 1/6 segments

        let single_digit_segments = (0..=9)
            .map(Segments::digit_to_segments)
            .map(Result::unwrap)
            .collect::<Vec<_, 10>>();
        let segment_display_time = Duration::from_millis(130);
        let wait = segment_display_time * 2;
        let spin_wait = Duration::MIN; // none

        let min_frame_ms = 35;
        let max_frame_bonus = 120;
        const FRAMES: usize = 4 * 6; // full circle
        let deceleration = |t: f32| {
            Duration::from_millis(min_frame_ms + (t.powf(2.75f32) * max_frame_bonus as f32) as u64)
        };
        let acceleration = |t: f32| deceleration(1f32 - t);

        loop {
            // go up
            defmt::info!("Increasing...");
            let mut count_up_ticker = Ticker::every(segment_display_time);
            for (i, &up) in single_digit_segments.iter().enumerate() {
                self.display_segments(&SegmentsWithDot(up, (i >= 5).into()));
                count_up_ticker.next().await;
            }
            
            // reached up
            defmt::info!("Waiting TOP...");
            self.spin_animate::<FRAMES>(&a_pattern, false, On, deceleration, spin_wait)
                .await;
            self.display_segments(&SegmentsWithDot(Segments::ONLY_G, On));
            Timer::after(wait).await;
            self.spin_animate::<FRAMES>(&a_pattern, true, On, acceleration, spin_wait)
                .await;

            // go down
            defmt::info!("Decreasing...");
            let mut count_down_ticker = Ticker::every(segment_display_time);
            for (i, &down) in single_digit_segments.iter().enumerate().rev() {
                self.display_segments(&SegmentsWithDot(down, (i >= 5).into()));
                count_down_ticker.next().await;
            }
            
            // reached down
            defmt::info!("Waiting BOT...");
            self.spin_animate::<FRAMES>(&a_pattern, true, Off, deceleration, spin_wait)
                .await;
            self.display_segments(&SegmentsWithDot(Segments::ONLY_G, Off));
            Timer::after(wait).await;
            self.spin_animate::<FRAMES>(&a_pattern, false, Off, acceleration, spin_wait)
                .await;
        }
    }
    /// Animate a loop from display pin A to F (not including G which is the middle).
    /// A loop is composed of the 6 outer segments, and the spin can be animated by creating a cyclic window iterator and yielding it an amount of times.
    async fn spin_animate<const FRAMES: usize>(
        &mut self,
        pattern: &[ActivationState; 6],
        counter_clockwise: bool,
        dot: ActivationState,
        delay_fn: impl Fn(f32) -> Duration,
        wait_time: Duration,
    ) {
        use ActivationState::*;
        self.clear_display();
        Timer::after(wait_time).await;
        let counter_clockwise_rotation = pattern
            .iter()
            .copied()
            .cycle()
            .map_windows(|a: &[ActivationState; 6]| {
                a.iter()
                    .copied()
                    .chain(core::iter::once(Off))
                    .collect::<Vec<_, 7>>()
                    .into_array()
                    .unwrap()
            })
            .take(FRAMES)
            .collect::<Vec<_, FRAMES>>();

        let directional_pattern: Either<_, _> = if counter_clockwise {
            Either::Left(counter_clockwise_rotation.iter())
        } else {
            Either::Right(counter_clockwise_rotation.iter().rev())
        };
        for (delay, &iteration) in directional_pattern
            .enumerate()
            .map(|(i, arr)| (delay_fn(i as f32 / FRAMES as f32), arr))
        {
            self.display_segments(&SegmentsWithDot(Segments(iteration), dot));
            Timer::after(delay).await;
        }

        self.clear_display();
        Timer::after(wait_time).await;
    }
}

// would use impl trait / generics constraint if it was allowed for embassy tasks
#[embassy_executor::task]
pub(crate) async fn cycle_numbers_cathode(mut target: ShiftRegisterSIPO<'static, 8>) {
    SevenSegmentedDisplayWithDot::<CommonCathode>::cycle_numbers(&mut target).await;
}
