use crate::characters::{BitDisplayable, DecimalDigit, DisplayLetter};
use arduino_hal::port::mode::Output;
use arduino_hal::port::{Pin, PinOps};
use core::iter::Cycle;
use core::ops::Range;
use embedded_hal::digital::{OutputPin, PinState};

#[allow(unused)] // for printing
use {
    crate::print::{println, CONSOLE},
    avr_device::interrupt,
};

#[derive(Debug)]
#[allow(unused)]
pub enum Common {
    Anode,
    Cathode,
}
impl core::ops::Not for Common {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Common::Anode => Common::Cathode,
            Common::Cathode => Common::Anode,
        }
    }
}
impl Common {
    pub(crate) fn on_state(&self) -> PinState {
        match self {
            Common::Anode => PinState::Low,
            Common::Cathode => PinState::High,
        }
    }
    pub(crate) fn off_state(&self) -> PinState {
        match self {
            Common::Anode => PinState::High,
            Common::Cathode => PinState::Low,
        }
    }
}

#[allow(unused)]
pub trait Toggle<S>
where
    Self: Sized,
{
    fn set_all(&mut self, state: S);
    fn set_all_on(&mut self);
    fn set_all_off(&mut self);
    fn with_all_on(mut self) -> Self {
        self.set_all_on();
        self
    }
    fn with_all_off(mut self) -> Self {
        self.set_all_off();
        self
    }
    fn with_all(mut self, state: S) -> Self {
        self.set_all(state);
        self
    }
}

pub struct SevenSegmentPlusDot<
    A: PinOps,
    B: PinOps,
    C: PinOps,
    D: PinOps,
    E: PinOps,
    F: PinOps,
    G: PinOps,
    DP: PinOps,
> {
    pub a: Pin<Output, A>,
    // top bar
    pub b: Pin<Output, B>,
    // upper right
    pub c: Pin<Output, C>,
    // lower right
    pub d: Pin<Output, D>,
    // bottom bar
    pub e: Pin<Output, E>,
    // lower left
    pub f: Pin<Output, F>,
    // upper left
    pub g: Pin<Output, G>,
    // middle bar
    pub dp: Pin<Output, DP>,
    // led state type
    pub common: Common,
}

impl<A: PinOps, B: PinOps, C: PinOps, D: PinOps, E: PinOps, F: PinOps, G: PinOps, DP: PinOps>
    Toggle<PinState> for SevenSegmentPlusDot<A, B, C, D, E, F, G, DP>
{
    fn set_all(&mut self, state: PinState) {
        [
            self.c.set_state(state),
            self.d.set_state(state),
            self.e.set_state(state),
            self.f.set_state(state),
            self.g.set_state(state),
            self.b.set_state(state),
            self.a.set_state(state),
            self.dp.set_state(state),
        ]
        .into_iter()
        .collect::<Result<(), _>>()
        .expect("infallible?");
    }
    fn set_all_on(&mut self) {
        self.set_all(self.common.on_state())
    }
    fn set_all_off(&mut self) {
        self.set_all(self.common.off_state())
    }
}

impl<A: PinOps, B: PinOps, C: PinOps, D: PinOps, E: PinOps, F: PinOps, G: PinOps, DP: PinOps>
    SevenSegmentPlusDot<A, B, C, D, E, F, G, DP>
{
    #[allow(unused)]
    pub fn test_segments_one_by_one(&mut self) {
        let on = self.common.on_state();
        let off = self.common.off_state();
        self.a.set_state(on).unwrap();
        arduino_hal::delay_ms(1000);
        self.a.set_state(off).unwrap();
        self.b.set_state(on).unwrap();
        arduino_hal::delay_ms(1000);
        self.b.set_state(off).unwrap();
        self.c.set_state(on).unwrap();
        arduino_hal::delay_ms(1000);
        self.c.set_state(off).unwrap();
        self.d.set_state(on).unwrap();
        arduino_hal::delay_ms(1000);
        self.d.set_state(off).unwrap();
        self.e.set_state(on).unwrap();
        arduino_hal::delay_ms(1000);
        self.e.set_state(off).unwrap();
        self.f.set_state(on).unwrap();
        arduino_hal::delay_ms(1000);
        self.f.set_state(off).unwrap();
        self.g.set_state(on).unwrap();
        arduino_hal::delay_ms(1000);
        self.g.set_state(off).unwrap();
        self.dp.set_state(on).unwrap();
        arduino_hal::delay_ms(1000);
        self.dp.set_state(off).unwrap();
    }

    #[allow(unused)]
    pub fn set_digit(&mut self, digit: u8) {
        let digit = DecimalDigit::try_from(digit)
            .expect("only handles actually displayable numbers (digits)");
        self.set_displayable(digit)
    }

    #[allow(unused)]
    pub fn set_char(&mut self, c: char) {
        let c = DisplayLetter::try_from(c).expect("only handles displayable characters");
        self.set_displayable(c);
    }

    pub fn set_displayable(&mut self, displayable: impl BitDisplayable) {
        let bits = displayable.displayable_bits();
        self.set_bits(bits);
    }

    #[allow(unused)]
    pub fn clear(&mut self) {
        self.set_bits(0b0);
    }

    pub fn set_bits(&mut self, bits: u8) {
        // println!("- Bits: {:x}", bits);
        let bit_state = |i: u8| {
            let shifted = 1 << i;
            let and_ed = bits & shifted;
            if and_ed != 0 {
                self.common.on_state()
            } else {
                self.common.off_state()
            }
        };
        [
            self.a.set_state(bit_state(0)),
            self.b.set_state(bit_state(1)),
            self.c.set_state(bit_state(2)),
            self.d.set_state(bit_state(3)),
            self.e.set_state(bit_state(4)),
            self.f.set_state(bit_state(5)),
            self.g.set_state(bit_state(6)),
        ]
        .into_iter()
        .collect::<Result<(), _>>()
        .expect("infallible");
    }
}

pub struct FourDigitPins<D1: PinOps, D2: PinOps, D3: PinOps, D4: PinOps> {
    // display_state: PinState,
    pub d1: Pin<Output, D1>,
    pub d2: Pin<Output, D2>,
    pub d3: Pin<Output, D3>,
    pub d4: Pin<Output, D4>,
    pub common: Common,
}
impl<D1: PinOps, D2: PinOps, D3: PinOps, D4: PinOps> Toggle<PinState>
    for FourDigitPins<D1, D2, D3, D4>
{
    fn set_all(&mut self, state: PinState) {
        [
            self.d1.set_state(state),
            self.d2.set_state(state),
            self.d3.set_state(state),
            self.d4.set_state(state),
        ]
        .into_iter()
        .collect::<Result<(), _>>()
        .expect("infallible");
    }

    fn set_all_on(&mut self) {
        self.set_all(self.common.on_state());
    }

    fn set_all_off(&mut self) {
        self.set_all(self.common.off_state());
    }
}

impl<D1: PinOps, D2: PinOps, D3: PinOps, D4: PinOps> FourDigitPins<D1, D2, D3, D4> {
    fn set_single_state(&mut self, state: PinState, id: u8) {
        match id {
            0 => self.d1.set_state(state),
            1 => self.d2.set_state(state),
            2 => self.d3.set_state(state),
            3 => self.d4.set_state(state),
            _ => unimplemented!("This structure only handles four digits specifically."),
        }
        .expect("infallible");
    }
    #[allow(unused)]
    fn set_single_on(&mut self, id: u8) {
        self.set_single_state(self.common.on_state(), id);
    }
    #[allow(unused)]
    fn set_single_off(&mut self, id: u8) {
        self.set_single_state(self.common.on_state(), id);
    }
}

/// Attempt at representing a 4-digit 7-segment LED.
/// Display digit pins (D1-4) can be set to high to enable them,
/// setting them to the state dictated by the remaining pins (A-DP).
/// https://softwareparticles.com/learn-how-a-4-digit-7-segment-led-display-works-and-how-to-control-it-using-an-arduino/
///
/// ### Simultaneous multiplexing
/// Without a proper hardware display driver, we can emulate the effect using "persistence of vision",
/// meaning by changing which digit is turned on *very* fast.
/// There are two ways of achieving this (AFAIK):
/// - Rotating through digit pins with unique segments pins turned on at the right time
/// - Rotating through segments pins very fast and turning on the right digits pins at the right time
/// The former is simpler from an API perspective, since rotating through each digit is easy.
/// It also allows us to have up to a fourth of all the visible segments in the display active at the same time.
/// The second approach rotates slower in the first loop (7+1 segments in total vs. 4 digits),
/// but may make up for it since you are quite likely to use the same segment across multiple digits at the same time,
/// whereas you are less likely to want to display the same digit over multiple displays.
///
/// ### Type parameters
/// Different generics are needed because each pin is a different concrete type.
pub struct FourDigitDisplay<
    A: PinOps,
    B: PinOps,
    C: PinOps,
    D: PinOps,
    E: PinOps,
    F: PinOps,
    G: PinOps,
    DP: PinOps,
    D1: PinOps,
    D2: PinOps,
    D3: PinOps,
    D4: PinOps,
    I: Iterator,
> {
    pub segments: SevenSegmentPlusDot<A, B, C, D, E, F, G, DP>,
    pub digit_pins: FourDigitPins<D1, D2, D3, D4>,
    pub num: u16,
    pub digit_rotation: Cycle<I>,
}

pub const ROTATION: Range<u8> = 0..4;

pub const DISPLAY_DELAY: u16 = 1;

#[allow(unused)]
impl<
        A: PinOps,
        B: PinOps,
        C: PinOps,
        D: PinOps,
        E: PinOps,
        F: PinOps,
        G: PinOps,
        DP: PinOps,
        D1: PinOps,
        D2: PinOps,
        D3: PinOps,
        D4: PinOps,
        I: Iterator<Item = u8> + Clone,
    > FourDigitDisplay<A, B, C, D, E, F, G, DP, D1, D2, D3, D4, I>
{
    /// Every digit in a number can be isolated by starting from the left (most significant digit) and going right:
    /// - Use modulus on the highest power of ten: `num % 10^p`,
    /// where `p` (position/place) is equal to the number of digits in the display e.g. 4: `num % 10^4`.
    /// This removes number information above digit from source number.
    /// - Divide by `10^(p-1)` to get the digit in itself,
    /// getting the digit without the number information on the right,
    /// (for the last digit in number, this step is irrelevant).
    /// - Repeat for each step in digit sequence, while `p > 0` e.g. duration of `p..0` or `1..=p`
    /// No need for any carry because of the first step.
    pub fn keep_active_for_ms(&mut self) {
        let next_p = self.digit_rotation.next().expect("cyclic");
        let self_place_value = 10u16.pow(next_p as u32);
        let left_place_value = self_place_value * 10; // one place higher
        let self_and_below = self.num % left_place_value;
        let self_digit_isolated = self_and_below / self_place_value;
        self.digit_pins.set_all_off();
        self.digit_pins.set_single_on(next_p);
        self.segments.set_digit(self_digit_isolated as u8);
        arduino_hal::delay_ms(DISPLAY_DELAY);
        self.digit_pins.set_all_off();
    }
}
