use base64::{engine::general_purpose, Engine as _};
use mime_guess::from_path;
use reqwest::{get, Error};

pub async fn encode_image_from_url(url: &str) -> Result<String, Error> {
    let image_response = get(url).await?;
    let bytes = image_response.bytes().await?;
    let mime = from_path(url).first_or_octet_stream().to_string();

    Ok(general_purpose::STANDARD_NO_PAD.encode(&bytes))
        .map(|encoded| format!("data:{};base64,{}", mime, encoded))
}
