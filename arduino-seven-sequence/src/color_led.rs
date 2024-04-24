#![allow(unused)]

use core::iter::Cycle;

use arduino_hal::port::mode::PwmOutput;
use arduino_hal::port::Pin;

#[derive(Debug, Clone)]
pub struct Color<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

pub struct ColorPins<TCR, TCG, TCB, R, G, B> {
    pub r: Pin<PwmOutput<TCR>, R>,
    pub g: Pin<PwmOutput<TCG>, G>,
    pub b: Pin<PwmOutput<TCB>, B>,
}

pub struct StatefulLED<TCR, TCG, TCB, R, G, B, I: Iterator<Item = Color<u8>> + Clone> {
    pub led: ColorPins<TCR, TCG, TCB, R, G, B>,
    pub cyclic_state: Cycle<I>,
}
