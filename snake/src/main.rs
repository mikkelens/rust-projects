use console_engine::{pixel, KeyCode};

// enum
#[allow(clippy::cast_possible_wrap)]
fn main() {
	let mut engine = console_engine::ConsoleEngine::init_fill_require(50, 20, 5).unwrap();

	loop {
		engine.wait_frame();
		engine.check_resize();

		let mut input: Vec<i32> = vec![0, 0];
		if engine.is_key_pressed(KeyCode::Char('w')) {
			input[1] += 1;
		}
		if engine.is_key_pressed(KeyCode::Char('s')) {
			input[1] -= 1;
		}
		if engine.is_key_pressed(KeyCode::Char('a')) {
			input[0] -= 1;
		}
		if engine.is_key_pressed(KeyCode::Char('d')) {
			input[0] += 1;
		}
		engine.clear_screen();

		engine.fill_rect(
			0,
			0,
			engine.get_width() as i32,
			engine.get_height() as i32,
			pixel::pxl('░')
		);

		engine.draw();
	}
}
