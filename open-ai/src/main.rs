use async_openai::{ types::{ CreateImageRequestArgs, ImageSize, ResponseFormat }, Client };
use linefeed::{ Interface, ReadResult };
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let reader = Interface::new("my-application")?;
    reader.set_prompt("> ")?;

    println!("Give an image prompt.");
    let input = loop { // until valid value is received from user
        if let ReadResult::Input(valid_value) = reader.read_line()? {
            break valid_value;
        }
        print!("Invalid answer!");
    };

    let client = Client::new(); // works only if environment variable OPENAI_API_KEY is set
    let request = CreateImageRequestArgs::default()
        .prompt(input)
        .n(1)
        .response_format(ResponseFormat::Url)
        .size(ImageSize::S256x256)
        .user("async-openai")
        .build()?;

    let response = client.images().create(request).await?;
    let paths = response.save("./data").await?;

    paths.iter().for_each(|path| {
        println!("Image file path: {}", path.display());
        open::that(path).ok();
    });

    Ok(())
}