use crate::err::RacetimeError;
use bytes::Bytes;
use http::Response;
use reqwest::{Client as ReqwestClient, ClientBuilder};
use thiserror::Error;
use url::{ParseError, Url};

const BASE_URL: &str = "https://racetime.gg/";

#[derive(Debug)]
pub struct RacetimeClient {
    client: ReqwestClient,
    endpoint: Url,
}

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Url parsing error: {0}")]
    UrlError(#[from] ParseError),
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] http::Error),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

impl RacetimeClient {
    pub fn new() -> Result<Self, ClientError> {
        let client = ClientBuilder::new().build()?;
        let endpoint = Url::parse(BASE_URL)?;
        Ok(Self { client, endpoint })
    }

    pub async fn rest_async(&self, url: Url) -> Result<Response<Bytes>, RacetimeError> {
        let request = self.client.get(url).build()?;
        let rsp = self.client.execute(request).await?;

        let mut http_rsp = http::Response::builder()
            .status(rsp.status())
            .version(rsp.version());
        let headers = http_rsp.headers_mut().unwrap();
        for (key, val) in rsp.headers() {
            headers.insert(key, val.clone());
        }
        http_rsp.body(rsp.bytes().await?).map_err(From::from)
    }

    pub(crate) fn rest_endpoint(&self, endpoint: &str) -> Result<Url, RacetimeError> {
        self.endpoint
            .join(endpoint.trim_start_matches('/'))
            .map_err(From::from)
    }
}
