#[cfg(test)]
mod tests {
    use crate::utils::url::is_valid_url;

    #[test]
    fn test_valid_url() {
        assert!(is_valid_url("https://www.example.com"));
        assert!(is_valid_url("https://www.example.xyz"));
        assert!(is_valid_url("https://example.xyz"));
        assert!(is_valid_url("https://www.example.be"));
        assert!(is_valid_url("https://www.com"));
        assert!(is_valid_url("https://example.be"));
        assert!(is_valid_url("http://localhost:8080/test"));
    }

    #[test]
    fn test_invalid_url() {
        assert!(!is_valid_url("https://"));
        assert!(!is_valid_url("not_a_valid_url"));
    }
}
