use embassy_rp::gpio::Level;

/// meant for use with enums
pub(crate) trait LEDState: From<Level> + Into<Level> + Copy + Sized {
    fn default_on() -> Self;
    fn off() -> Self {
        Level::High.into()
    }
    fn is_on(&self) -> bool {
        (*self).into() == Level::High
    }
    fn is_off(&self) -> bool {
        (*self).into() == Level::Low
    }
}

///// assume RED only
//pub(crate) trait MatrixLED<const ROWS: usize, const COLUMNS: usize, S: LEDState> {
//    type Image = [[S; ROWS]; COLUMNS];
//    fn last_image(&self) -> &Self::Image;
//    fn scanline_iter(&self) -> &[impl LEDState] {
//        self.last_image().as_flattened()
//    }
//    fn set_image(&mut self, image: Self::Image);
//    async fn run_endlessly(&mut self);
//    async fn run_for(&mut self, duration: Duration) {}
//}
