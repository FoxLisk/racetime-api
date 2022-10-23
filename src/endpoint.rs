use crate::client::RacetimeClient;
use crate::err::RacetimeError;
use async_trait::async_trait;
use http::StatusCode;
use serde::de::DeserializeOwned;
use std::borrow::Cow;

/// Endpoint exposed by racetime.gg
pub trait Endpoint {
    fn endpoint(&self) -> Cow<'static, str>;

    fn set_query_parameters(&self, url: &mut url::Url) -> Result<(), RacetimeError> {
        let query = self.query_parameters()?;
        url.set_query(Some(query.as_ref()));
        Ok(())
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, RacetimeError> {
        Ok(Cow::default())
    }
}

/// Query made to a client
#[async_trait]
pub trait Query<T> {
    /// Perform an asynchronous query against the client.
    async fn query(&self, client: &RacetimeClient) -> Result<T, RacetimeError>;
}

#[async_trait]
impl<E, T> Query<T> for E
where
    E: Endpoint + Sync,
    T: DeserializeOwned + 'static,
{
    async fn query(&self, client: &RacetimeClient) -> Result<T, RacetimeError> {
        let mut url = client.rest_endpoint(&self.endpoint())?;
        self.set_query_parameters(&mut url)?;

        let rsp = client.rest_async(url).await?;
        let status = rsp.status();

        if status == StatusCode::NOT_FOUND {
            return Err(RacetimeError::NotFound);
        }

        if !status.is_success() {
            return Err(RacetimeError::UnexpectedStatus(status));
        }

        serde_json::from_slice(rsp.body()).map_err(From::from)
    }
}
