use async_openai::{
    types::{CreateImageRequestArgs, ImageSize, ResponseFormat},
    Client,
};
use linefeed::{Interface, ReadResult};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let reader = Interface::new("my-application")?;
    reader.set_prompt("> ")?;
    
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

    paths
        .iter()
        .for_each(|path| {
            println!("Image file path: {}", path.display());
            open::that(path).ok();
        });

    Ok(())
}
