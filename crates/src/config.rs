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

        let config = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;
        let config = config.try_deserialize::<Config>()?;

        assert_eq!(config.url, "localhost");

        Ok(())
    }
}
