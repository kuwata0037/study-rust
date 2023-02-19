pub mod crawler;

use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::Name;
use thiserror::Error;
use url::ParseError as UrlParseError;
use url::Url;

#[derive(Error, Debug)]
pub enum GetLinkError {
    #[error("Failed to send a request")]
    SendRequest(#[source] reqwest::Error),
    #[error("Failed to read the response body")]
    ResponseBody(#[source] reqwest::Error),
    #[error("Server returned an error")]
    ServerError(#[source] reqwest::Error),
}

pub struct LinkExtractor {
    client: Client,
}

impl LinkExtractor {
    pub fn from_client(client: Client) -> Self {
        Self { client }
    }

    pub fn get_links(&self, url: Url) -> Result<Vec<Url>, GetLinkError> {
        log::info!("GET \"{}\"", url);
        let response = self
            .client
            .get(url)
            .send()
            .map_err(GetLinkError::SendRequest)?
            .error_for_status()
            .map_err(GetLinkError::ServerError)?;
        let base_url = response.url().clone();
        let status = response.status();
        let body = response.text().map_err(GetLinkError::ResponseBody)?;
        let doc = Document::from(body.as_str());

        let mut links = Vec::new();
        log::info!("Retrieved {} \"{}\"", status, base_url);
        for href in doc.find(Name("a")).filter_map(|a| a.attr("href")) {
            match Url::parse(href) {
                Ok(mut url) => {
                    url.set_fragment(None);
                    links.push(url);
                }
                Err(UrlParseError::RelativeUrlWithoutBase) => match base_url.join(href) {
                    Ok(mut url) => {
                        url.set_fragment(None);
                        links.push(url);
                    }
                    Err(e) => {
                        log::warn!("URL join error: {}", e);
                    }
                },
                Err(e) => {
                    println!("{}", e);
                }
            }
        }

        Ok(links)
    }
}

impl crawler::AdjacentNodes for LinkExtractor {
    type Node = Url;

    fn adjacent_nodes(&self, v: &Self::Node) -> Vec<Self::Node> {
        match self.get_links(v.clone()) {
            Ok(links) => links,
            Err(e) => {
                use std::error::Error;

                log::warn!("Error occurred: {}", e);

                let mut e = e.source();
                while let Some(err) = e {
                    log::warn!("Error source: {}", err);
                    e = err.source();
                }

                vec![]
            }
        }
    }
}
