#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct Config {
        pub url: String,
    }

    #[test]
    fn test_config() -> Result<(), config::ConfigError> {
        dotenv().ok();

        let mut config = config::Config::default();
        config.merge(config::Environment::new())?;

        let config = config.try_into::<Config>()?;
        assert_eq!(config.url, "localhost");

        Ok(())
    }
}
