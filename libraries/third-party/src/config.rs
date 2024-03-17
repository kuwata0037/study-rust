#[cfg(test)]
mod tests {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct Config {
        url: String,
    }

    #[test]
    fn test_config() -> Result<(), config::ConfigError> {
        std::env::set_var("url", "localhost");

        let config = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;
        let config = config.try_deserialize::<Config>()?;

        assert_eq!(config.url, "localhost");

        Ok(())
    }
}
