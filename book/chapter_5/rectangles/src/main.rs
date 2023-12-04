use crate::structs::Rectangle;

mod structs;

fn main() {
    let thin_rect = Rectangle {
        width: 10,
        height: 40,
    };
    let giant_rect = Rectangle {
        width: 45,
        height: 70,
    };

    println!("Can giant_rect hold thin_rect? {}!",
             giant_rect.can_hold(&thin_rect));
}