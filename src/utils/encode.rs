use base64::{engine::general_purpose, Engine as _};
use mime_guess::from_path;
use reqwest::get;
use tracing::error;

use super::url::is_valid_url;

pub async fn encode_image_from_url(url: &str) -> Result<String, String> {
    if !is_valid_url(url) {
        return Err("Invalid URL".to_string());
    }

    let response = match get(url).await {
        Ok(response) => response,
        Err(e) => {
            error!("Error fetching image: {:?}", e);
            return Err("Failed to fetch image".to_string());
        }
    };
    let bytes = match response.bytes().await {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Error reading image: {:?}", e);
            return Err("Failed to read image".to_string());
        }
    };
    let mime = from_path(url).first_or_octet_stream();

    if mime.type_() != "image" {
        error!("Invalid mime type: {:?}", mime.type_());
        return Err("Invalid mime type".to_string());
    }

    let encoded = general_purpose::STANDARD_NO_PAD.encode(&bytes);

    Ok(format!(
        "data:{};base64,{}",
        mime.to_string(),
        encoded.to_string()
    ))
}
