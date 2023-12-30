use base64::{engine::general_purpose, Engine as _};
use mime_guess::from_path;
use reqwest::get;
use tracing::error;

pub async fn encode_image_from_url(url: &str) -> Result<String, String> {
    match get(url).await {
        Ok(response) => match response.bytes().await {
            Ok(bytes) => {
                let mime = from_path(url).first_or_octet_stream().to_string();

                Ok(general_purpose::STANDARD_NO_PAD.encode(&bytes))
                    .map(|encoded| format!("data:{};base64,{}", mime, encoded))
            }
            Err(e) => {
                error!("Error fetching data: {:?}", e);
                Ok("".to_string())
            }
        },
        Err(e) => {
            error!("Error fetching data: {:?}", e);
            Ok("".to_string())
        }
    }
}
