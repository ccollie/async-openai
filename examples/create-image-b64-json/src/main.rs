use std::error::Error;

use async_openai as openai;
use openai::{
    types::{CreateImageRequest, ImageSize, ResponseFormat},
    Client, Image,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // create client, reads OPENAI_API_KEY environment variable for API key.
    let client = Client::new();

    let request = CreateImageRequest {
        prompt: "Generate a logo for github repository async-openai".to_owned(),
        n: Some(2),
        response_format: Some(ResponseFormat::B64Json),
        size: Some(ImageSize::S256x256),
        user: Some("async-openai".to_owned()),
    };

    let response = Image::create(&client, request).await?;

    // download and save images to ./data directory
    // (creates directory when it doesn't exist)
    response.save("./data").await?;

    Ok(())
}
