use reqwest::blocking::ClientBuilder;
use url::Url;
use web_crawler::LinkExtractor;

fn main() -> eyre::Result<()> {
    env_logger::init();

    let arg = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "https://www.rust-lang.org".to_string());
    let url = Url::parse(arg.as_str())?;
    let client = ClientBuilder::new().build()?;
    let extractor = LinkExtractor::from_client(client);

    let links = extractor.get_links(url)?;
    for link in &links {
        println!("{}", link);
    }

    Ok(())
}
