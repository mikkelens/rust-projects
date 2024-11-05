mod expressions;
mod maps;

use expressions::*;
use maps::*;

fn main() {
	let arg = std::env::args().nth(1).unwrap();
	let expression = arg.parse::<ExprNode>();
	println!("Exp (debug):\n{:?}\n", expression);
	if let Ok(expr) = expression {
		println!("Exp (display):\n{}\n", expr);
		let map = Map::from(expr);
		println!("Map:\n{}\n", map);
	}
}
