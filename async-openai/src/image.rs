use reqwest::Body;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::{
    error::OpenAIError,
    types::{
        CreateImageEditRequest, CreateImageRequest, CreateImageVariationRequest, ImageInput,
        ImageResponse,
    },
    Client,
};

/// Given a prompt and/or an input image, the model will generate a new image.
///
/// Related guide: [Image generation](https://beta.openai.com/docs/guides/images/introduction)
pub struct Image;

impl Image {
    /// Creates an image given a prompt.
    pub async fn create(
        client: &Client,
        request: CreateImageRequest,
    ) -> Result<ImageResponse, OpenAIError> {
        client.post("/images/generations", request).await
    }

    pub(crate) async fn file_stream_body(image_input: &ImageInput) -> Result<Body, OpenAIError> {
        let file = tokio::fs::File::open(image_input.path.as_path())
            .await
            .map_err(|e| OpenAIError::ImageReadError(e.to_string()))?;
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        Ok(body)
    }

    /// Creates the part for the given image file for multipart upload.
    pub(crate) async fn create_part(
        image_input: &ImageInput,
    ) -> Result<reqwest::multipart::Part, OpenAIError> {
        let image_file_name = image_input
            .path
            .as_path()
            .file_name()
            .ok_or_else(|| {
                OpenAIError::ImageReadError(format!(
                    "cannot extract file name from {:#?}",
                    image_input.path
                ))
            })?
            .to_str()
            .unwrap()
            .to_string();

        let image_part =
            reqwest::multipart::Part::stream(Image::file_stream_body(image_input).await?)
                .file_name(image_file_name)
                .mime_str("application/octet-stream")
                .unwrap();

        Ok(image_part)
    }

    /// Creates an edited or extended image given an original image and a prompt.
    pub async fn create_edit(
        client: &Client,
        request: CreateImageEditRequest,
    ) -> Result<ImageResponse, OpenAIError> {
        let image_part = Image::create_part(&request.image).await?;
        let mask_part = Image::create_part(&request.mask).await?;

        let mut form = reqwest::multipart::Form::new()
            .part("image", image_part)
            .part("mask", mask_part)
            .text("prompt", request.prompt);

        if request.n.is_some() {
            form = form.text("n", request.n.unwrap().to_string())
        }

        if request.size.is_some() {
            form = form.text("size", request.size.unwrap().to_string())
        }

        if request.response_format.is_some() {
            form = form.text(
                "response_format",
                request.response_format.unwrap().to_string(),
            )
        }

        if request.user.is_some() {
            form = form.text("user", request.user.unwrap())
        }

        client.post_form("/images/edits", form).await
    }

    /// Creates a variation of a given image.
    pub async fn create_variation(
        client: &Client,
        request: CreateImageVariationRequest,
    ) -> Result<ImageResponse, OpenAIError> {
        let image_part = Image::create_part(&request.image).await?;

        let mut form = reqwest::multipart::Form::new().part("image", image_part);

        if request.n.is_some() {
            form = form.text("n", request.n.unwrap().to_string())
        }

        if request.size.is_some() {
            form = form.text("size", request.size.unwrap().to_string())
        }

        if request.response_format.is_some() {
            form = form.text(
                "response_format",
                request.response_format.unwrap().to_string(),
            )
        }

        if request.user.is_some() {
            form = form.text("user", request.user.unwrap())
        }

        client.post_form("/images/variations", form).await
    }
}
