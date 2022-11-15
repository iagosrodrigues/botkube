pub mod config;

#[cfg(test)]
mod tests {
    use super::config::*;

    #[test]
    fn validate_configuration() {
        let config = Config::load_from_file("./tests/Configuration.toml").unwrap();

        assert_eq!(config.communications["first_group"].telegram.enabled, true);

        if let Some(token) = &config.communications["first_group"].telegram.token {
            assert_eq!(token, "token");
        }

        if let Some(chat_id) = &config.communications["first_group"].telegram.chat_id {
            assert_eq!(chat_id, "chat_id");
        }
    }

    #[test]
    fn test_not_found_configuration() {
        let config = Config::load_from_file("./tests/not_found.toml");

        assert_eq!(config.err().is_none(), false);
    }
}
