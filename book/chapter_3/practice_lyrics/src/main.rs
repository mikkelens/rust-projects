use crate::strings::{DATE_NAMES, NEW_DAY_GIFTS};

mod strings;

fn main() {
	for (day, name) in DATE_NAMES.iter().enumerate() {
		// counting up
		println!("\nOn the {} day of Christmas,", name);
		println!("My good friends brought to me:");

		for lyric in (0..=day).rev() {
			// counting down
			if day == 0 {
				// first gift alone
				println!("{}.", NEW_DAY_GIFTS[lyric]);
			} else if lyric > 0 {
				// most days
				println!("{},", NEW_DAY_GIFTS[lyric]);
			} else {
				// first gift as last sentence
				println!("And {}.", NEW_DAY_GIFTS[lyric]);
			}
		}
	}
}
