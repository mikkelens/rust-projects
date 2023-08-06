use anyhow::Result;
use async_openai::{
	types::{CreateImageRequestArgs, ImageSize, ResponseFormat},
	Client
};
use linefeed::{DefaultTerminal, Interface, ReadResult};

pub async fn gen_image(reader: &Interface<DefaultTerminal>) -> Result<()> {
	println!("Give an image prompt.");
	let input: String = match reader.read_line()? {
		ReadResult::Input(string) => string,
		_ => panic!("Invalid input!")
	};

	let client = Client::new();
	let request = CreateImageRequestArgs::default()
		.prompt(input)
		.n(1)
		.response_format(ResponseFormat::Url)
		.size(ImageSize::S256x256)
		.user("async-openai")
		.build()?;

	let response = client.images().create(request).await?;
	let paths = response.save("./data").await?;

	for path in &paths {
		println!("Image file path: {}", path.display());
		open::that(path).ok();
	}

	Ok(())
}
