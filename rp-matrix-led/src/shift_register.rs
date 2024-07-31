//use embassy_rp::gpio::{AnyPin, Level, Output};
//
///// Shift register that has many output pins from a single (serial) pin (and a few clock pins).
///// Typically, with a single shift register unit, the output size would be 8 bits/pins.
//#[allow(non_snake_case)]
//pub(crate) struct ShiftRegisterSIPO<'d, const PINS: usize> {
//    /// Serial (state/data)
//    pub(crate) SER: Output<'d, AnyPin>,
//    /// Shift Register Clock: set high to trigger input of serial state
//    pub(crate) SRCLK: Output<'d, AnyPin>,
//    /* optional pins: if None, assume physically ideal configuration? */
//    /// Storage Register Clock: set high to move from shift to storage (output new state)
//    pub(crate) RCLK: Option<Output<'d, AnyPin>>,
//    /// Shift Register Clock Reset (negative): connect and maintain *high* unless you want to reset
//    pub(crate) SRCLR: Option<Output<'d, AnyPin>>,
//    /// Output Enable (negative): connect and maintain *low* unless you want output disabled
//    pub(crate) OE: Option<Output<'d, AnyPin>>,
//}
//impl<'d, const PINS: usize> ShiftRegisterSIPO<'d, PINS> {
//    #[allow(unused)]
//    /// Clears out the values currently in the register
//    pub(crate) fn clear(&mut self) {
//        match &mut self.SRCLR {
//            Some(srclr) => {
//                srclr.set_low();
//            }
//            None => {
//                self.SRCLK.set_low();
//                for _ in 0..PINS {
//                    self.SRCLK.set_low();
//                    self.SRCLK.set_high();
//                    self.SRCLK.set_low();
//                }
//            }
//        }
//        self.latch();
//    }
//    // todo: abstract away from optional runtime behaviour into static trait implementation
//    pub(crate) fn disable_output(&mut self) {
//        if let Some(oe) = &mut self.OE {
//            oe.set_low();
//        }
//    }
//    pub(crate) fn enable_output(&mut self) {
//        if let Some(oe) = &mut self.OE {
//            oe.set_high();
//        }
//    }
//    /// Send/update output from internal state.
//    fn latch(&mut self) {
//        self.SRCLK.set_low(); // necessary?
//        if let Some(rclk) = &mut self.RCLK {
//            rclk.set_low(); // ?
//            rclk.set_high();
//            rclk.set_low();
//        }
//    }
//    /// Push bit to internal shift register
//    fn write_bit(&mut self, input_bit: Level) {
//        self.SER.set_level(input_bit);
//        self.SRCLK.set_low();
//        self.SRCLK.set_high();
//        self.SRCLK.set_low();
//    }
//    /// Push bit to shift register and output
//    pub(crate) fn set_next_bit(&mut self, input_bit: Level) {
//        self.write_bit(input_bit);
//        self.latch();
//    }
//    #[allow(unused)]
//    /// Push byte to shift register and output
//    pub(crate) fn set_next_byte(&mut self, input_byte: [Level; PINS]) {
//        self.SRCLK.set_low(); // ?
//        for bit in input_byte.iter().rev() {
//            self.write_bit(*bit);
//        }
//        self.latch();
//    }
//}

#![allow(unused)]

use embassy_rp::gpio::{AnyPin, Level, Output};

/// Basic data/clock pins
#[allow(non_snake_case)]
pub(crate) struct SerialOutput<'d> {
    pub SER: Output<'d, AnyPin>,
    pub SRCLK: Output<'d, AnyPin>,
}
/// Single Input, Parallel Output
trait Sipo {
    fn push_bit(&mut self, level: Level);
}
impl<'d> Sipo for SerialOutput<'d> {
    fn push_bit(&mut self, level: Level) {
        // order unsure
        self.SRCLK.set_low();
        self.SER.set_level(level);
        self.SRCLK.set_high();
        self.SRCLK.set_low();
    }
}
trait WriteSingle: Sipo {
    fn write_next(&mut self, level: Level) {
        self.push_bit(level); // non-latched
    }
}
impl<'d> WriteSingle for SerialOutput<'d> {}
trait WriteFull<const LEN: usize>: Sipo {
    fn write_full(&mut self, levels: [Level; LEN]) {
        for level in levels {
            self.push_bit(level);
        }
    }
}
impl<'d, const LEN: usize> WriteFull<LEN> for SerialOutput<'d> {}

/// Latched write implementation
trait Latched {
    fn latch(&mut self);
}
impl<T: Sipo + Latched> WriteSingle for T {
    fn write_next(&mut self, level: Level) {
        self.push_bit(level);
        self.latch();
    }
}
impl<T: Sipo + Latched, const LEN: usize> WriteFull<LEN> for T {
    fn write_full(&mut self, levels: [Level; LEN]) {
        for level in levels {
            self.push_bit(level);
        }
        self.latch();
    }
}

// The ability to turn something on/off
trait Toggleable {
    fn toggle(&mut self);
}
