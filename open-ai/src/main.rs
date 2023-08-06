mod chatgpt;
mod dalle;

use anyhow::Result;
use linefeed::{DefaultTerminal, Interface, ReadResult};

#[tokio::main]
async fn main() -> Result<()> {
	let reader = Interface::new("my-application")?;
	reader.set_prompt("> ")?;

	while let Some(c) = ask_type(&reader) {
		println!();
		match c {
			AIType::DallE => dalle::gen_image(&reader).await,
			AIType::ChatGPT => chatgpt::talk(&reader).await
		}?;
	} // runs untill we exit

	println!("Finished using OpenAI tool.");

	Ok(())
}

enum AIType {
	DallE,
	ChatGPT
}

// impl FromStr for Option<AIType> {
//     type Err = ();

//     fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
//         todo!()
//     }
// }

fn ask_type(reader: &Interface<DefaultTerminal>) -> Option<AIType> {
	let ReadResult::Input(user_input) = reader.read_line().ok()? else {
		return None;
	};

	if user_input.contains("chat") {
		return Some(AIType::ChatGPT);
	}
	if user_input.contains("dall") {
		return Some(AIType::DallE);
	}

	None
}
