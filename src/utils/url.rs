use url::Url;

pub fn is_valid_url(url: &str) -> bool {
    Url::parse(url).is_ok()
}
