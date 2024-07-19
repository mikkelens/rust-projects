use bitvec::order::Lsb0;
use bitvec::view::BitView;
use embassy_rp::gpio::{AnyPin, Level, Output};
use embassy_time::{Duration, Ticker};
use heapless::Vec;

//pub(crate) struct ShiftRegisterPISO<'d> {
//    serial_data: Input<'d, AnyPin>
//}

/// Shift register that has many output pins from a single (serial) pin (and a few clock pins).
/// Typically, with a single shift register unit, the output size would be 8 bits/pins.
#[allow(non_snake_case)]
pub(crate) struct ShiftRegisterSIPO<'d, const PINS: usize> {
    /// Serial (state/data)
    pub(crate) SER: Output<'d, AnyPin>,
    /// Shift Register Clock: set high to trigger input of serial state
    pub(crate) SRCLK: Output<'d, AnyPin>,
    /// Storage Register Clock: set high to move from shift to storage (output new state)
    pub(crate) RCLK: Output<'d, AnyPin>,
    /* optional pins */
    /// Shift Register Clock Reset (negative): connect and maintain *high* unless you want to reset
    pub(crate) SRCLR: Option<Output<'d, AnyPin>>,
    /// Output Enable (negative): connect and maintain *low* unless you want output disabled
    pub(crate) OE: Option<Output<'d, AnyPin>>,
}
impl<'d, const PINS: usize> ShiftRegisterSIPO<'d, PINS> {
    /// Clears out the values currently in the register
    pub(crate) fn clear(&mut self) {
        match &mut self.SRCLR {
            Some(srclr) => {
                srclr.set_low();
            }
            None => {
                self.SRCLK.set_low();
                for _ in 0..PINS {
                    self.SRCLK.set_low();
                    self.SRCLK.set_high();
                    self.SRCLK.set_low();
                }
            }
        }
        self.latch();
    }
    fn latch(&mut self) {
        self.SRCLK.set_low();
        self.RCLK.set_low(); // ?
        self.RCLK.set_high();
        self.RCLK.set_low();
    }
    /// push bit to shift register
    fn write_bit(&mut self, input_bit: Level) {
        self.SER.set_level(input_bit);
        self.SRCLK.set_low();
        self.SRCLK.set_high();
        self.SRCLK.set_low();
    }
    /// push byte to shift register
    pub(crate) fn write_full(&mut self, input_byte: &[Level; PINS]) {
        self.SRCLK.set_low(); // ?
        for bit in input_byte.iter().rev() {
            self.write_bit(*bit);
        }
        self.latch();
    }
}

type Scroll = u8;
const PINS: usize = size_of::<Scroll>();
#[embassy_executor::task]
pub(crate) async fn scroll(mut shift_register: ShiftRegisterSIPO<'static, PINS>) {
    let mut ticker = Ticker::every(Duration::from_millis(100));
    for i in {
        let range = 0..=Scroll::MAX; // reduce by 1?
        range.clone().chain(range.rev())
    }
    .cycle()
    {
        // seven_segment.display(i % 10, Level::High); // low is "on"
        shift_register.write_full(
            &i.view_bits::<Lsb0>()
                .into_iter()
                .map(|b| if *b { Level::High } else { Level::Low })
                .collect::<Vec<_, PINS>>()
                .into_array()
                .unwrap(),
        );

        // defmt::info!("displaying {}!", i);
        ticker.next().await;
    }
}
