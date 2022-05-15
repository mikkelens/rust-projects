#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

#[allow(dead_code)]
impl Rectangle {
    pub fn area(&self) -> u32 {
        // dbg!(rectangle);
        self.width * self.height
    }

    pub fn width(&self) -> bool {
        self.width > 0
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
