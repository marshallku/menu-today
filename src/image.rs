use base64::{engine::general_purpose, Engine as _};
use mime_guess::from_path;
use reqwest::get;

pub async fn get_image_data_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = get(url).await?;
    let bytes = response.bytes().await?;
    let mime = from_path(url).first_or_octet_stream().to_string();
    let encoded: String = general_purpose::STANDARD_NO_PAD.encode(&bytes);

    Ok(format!("data:{};base64,{}", mime, encoded))
}
