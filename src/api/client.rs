use std::error::Error;

use async_trait::async_trait;
use bytes::Bytes;
use http::request::Builder as RequestBuilder;
use http::Response;
use url::Url;

use crate::api::ApiError;
use crate::auth::Auth;

/// A trait representing a client which can communicate with a Marketstack instance via REST.
pub trait RestClient {
    /// The errors which may occur for this client.
    type Error: Error + Send + Sync + 'static;

    /// Get the URL for the endpoint for the client.
    ///
    /// This method adds the hostname for the client's target instance.
    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>>;

    /// Get the Auth token from the client.
    fn get_auth(&self) -> Option<Auth>;
}

/// A trait representing a client which can communicate with a Marketstack instance.
pub trait Client: RestClient {
    /// Send a REST query.
    fn rest(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}

/// A trait representing an asynchronous client which can communicate with a Marketstack instance.
#[async_trait]
pub trait AsyncClient: RestClient {
    /// Send a REST query asynchronously.
    async fn rest_async(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}
