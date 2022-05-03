use crate::strings::{DATE_NAMES, NEW_DAY_GIFTS};

mod strings;

fn main() {
    for day in 0..DATE_NAMES.len() { // counting up
        println!("\nOn the {} day of Christmas,", DATE_NAMES[day]);
        println!("My good friends brought to me:");

        for lyric in (0..=day).rev() { // counting down
            if day == 0 {
                // first gift alone
                println!("{}.", NEW_DAY_GIFTS[lyric])
            } else {
                if lyric > 0 {
                    // most days
                    println!("{},", NEW_DAY_GIFTS[lyric]);
                } else {
                    // first gift as last sentence
                    println!("And {}.", NEW_DAY_GIFTS[lyric]);
                }
            }
        }
    }
}
