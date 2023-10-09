use std::any;
use std::convert::TryInto;
use std::fmt::{self, Debug};

use async_trait::async_trait;
use bytes::Bytes;
use http::{request, HeaderMap, Response as HttpResponse};
use itertools::Itertools;
use log::{debug, error, info};
use reqwest::blocking::Client;
use reqwest::Client as AsyncClient;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use thiserror::Error;
use url::Url;

use crate::api;
use crate::auth::Auth;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RestError {
    #[error("communication with marketstack: {}", source)]
    Communication {
        #[from]
        source: reqwest::Error,
    },
    #[error("`http` error: {}", source)]
    Http {
        #[from]
        source: http::Error,
    },
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum MarketstackError {
    #[error("failed to parse url: {}", source)]
    UrlParse {
        #[from]
        source: url::ParseError,
    },
    #[error("communication with marketstack: {}", source)]
    Communication {
        #[from]
        source: reqwest::Error,
    },
    #[error("marketstack HTTP error: {}", status)]
    Http { status: reqwest::StatusCode },
    #[error("no response from marketstack")]
    NoResponse {},
    #[error("could not parse {} data from JSON: {}", typename, source)]
    DataType {
        #[source]
        source: serde_json::Error,
        typename: &'static str,
    },
    #[error("api error: {}", source)]
    Api {
        #[from]
        source: api::ApiError<RestError>,
    },
}

impl MarketstackError {
    fn http(status: reqwest::StatusCode) -> Self {
        MarketstackError::Http { status }
    }

    fn no_response() -> Self {
        MarketstackError::NoResponse {}
    }

    fn data_type<T>(source: serde_json::Error) -> Self {
        MarketstackError::DataType {
            source,
            typename: any::type_name::<T>(),
        }
    }
}

type MarketstackResult<T> = Result<T, MarketstackError>;

/// A representation of the Marketstack API.
#[derive(Clone)]
pub struct Marketstack {
    /// The client to use for API calls.
    client: Client,
    /// The base URL to use for API calls.
    rest_url: Url,
    /// The authentication information to use when communicating with Marketstack.
    auth: Auth,
}

impl Debug for Marketstack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Marketstack")
            .field("rest_url", &self.rest_url)
            .finish()
    }
}

impl Marketstack {
    /// Create a new Marketstack API representation.
    ///
    /// The `token` should be a valid [personal access token](https://marketstack.com/documentation).
    /// Errors out if `token` is invalid.
    pub fn new<H, T>(host: H, token: T) -> MarketstackResult<Self>
    where
        H: AsRef<str>,
        T: Into<String>,
    {
        Self::new_impl("https", host.as_ref(), Auth::Token(token.into()))
    }

    /// Create a new non-SSL Marketstack API representation.
    ///
    /// A `token` will still be required for insecure access.
    pub fn new_insecure<H, T>(host: H, token: T) -> MarketstackResult<Self>
    where
        H: AsRef<str>,
        T: Into<String>,
    {
        Self::new_impl("http", host.as_ref(), Auth::Token(token.into()))
    }

    /// Internal method to create a new Marketstack client.
    fn new_impl(protocol: &str, host: &str, auth: Auth) -> MarketstackResult<Self> {
        let rest_url = Url::parse(&format!("{}://{}/v1/", protocol, host))?;

        // NOTE: If cert validation is implemented / required, then add it here as `ClientCert`
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;

        let api = Marketstack {
            client,
            rest_url,
            auth,
        };

        // Ensure the API is working.
        api.auth.check_connection(&api)?;

        Ok(api)
    }

    /// Create a new Marketstack API client builder.
    pub fn builder<H, T>(host: H, token: T) -> MarketstackBuilder
    where
        H: Into<String>,
        T: Into<String>,
    {
        MarketstackBuilder::new(host, token)
    }

    fn send<T>(&self, req: reqwest::blocking::RequestBuilder) -> MarketstackResult<T>
    where
        T: DeserializeOwned,
    {
        let rsp = req.headers(HeaderMap::default()).send()?;
        let status = rsp.status();
        if status.is_server_error() {
            return Err(MarketstackError::http(status));
        }

        serde_json::from_reader::<_, T>(rsp).map_err(MarketstackError::data_type::<T>)
    }

    fn rest_simple(
        &self,
        mut request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, api::ApiError<<Self as api::RestClient>::Error>> {
        let call = || -> Result<_, RestError> {
            let http_request = request.body(body)?;
            let request = http_request.try_into()?;
            let rsp = self.client.execute(request)?;

            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            for (key, value) in rsp.headers() {
                headers.insert(key, value.clone());
            }
            Ok(http_rsp.body(rsp.bytes()?)?)
        };
        call().map_err(api::ApiError::client)
    }
}
pub struct MarketstackBuilder {
    protocol: &'static str,
    host: String,
    token: Auth,
}

impl MarketstackBuilder {
    /// Create a new Marketstack API client builder.
    pub fn new<H, T>(host: H, token: T) -> Self
    where
        H: Into<String>,
        T: Into<String>,
    {
        Self {
            protocol: "https",
            host: host.into(),
            token: Auth::Token(token.into()),
        }
    }

    /// Switch to an insecure protocol (http instead of https).
    pub fn insecure(&mut self) -> &mut Self {
        self.protocol = "http";
        self
    }

    pub fn build(&self) -> MarketstackResult<Marketstack> {
        Marketstack::new_impl(self.protocol, &self.host, self.token.clone())
    }

    pub async fn build_async(&self) -> MarketstackResult<AsyncMarketstack> {
        AsyncMarketstack::new_impl(self.protocol, &self.host, self.token.clone()).await
    }
}

impl api::RestClient for Marketstack {
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, api::ApiError<Self::Error>> {
        debug!(target: "marketstack", "REST api call {}", endpoint);
        Ok(self.rest_url.join(endpoint)?)
    }

    fn get_auth(&self) -> Option<Auth> {
        Some(self.auth.clone())
    }
}

impl api::Client for Marketstack {
    fn rest(
        &self,
        request: request::Builder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, api::ApiError<Self::Error>> {
        self.rest_simple(request, body)
    }
}

/// A represenation of the asynchronous Marketstack API.
#[derive(Clone)]
pub struct AsyncMarketstack {
    /// The client to use for API calls.
    client: reqwest::Client,
    /// The base URL to use for API calls.
    rest_url: Url,
    /// The authentication information to use when communicating with Marketstack.
    auth: Auth,
}

impl Debug for AsyncMarketstack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsyncMarketstack")
            .field("rest_url", &self.rest_url)
            .finish()
    }
}

#[async_trait]
impl api::RestClient for AsyncMarketstack {
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, api::ApiError<Self::Error>> {
        debug!(target: "marketstack", "REST api call {}", endpoint);
        Ok(self.rest_url.join(endpoint)?)
    }

    fn get_auth(&self) -> Option<Auth> {
        Some(self.auth.clone())
    }
}

#[async_trait]
impl api::AsyncClient for AsyncMarketstack {
    async fn rest_async(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, api::ApiError<<Self as api::RestClient>::Error>> {
        self.rest_async_simple(request, body).await
    }
}

impl AsyncMarketstack {
    /// Internal method to create a new Marketstack client.
    async fn new_impl(protocol: &str, host: &str, auth: Auth) -> MarketstackResult<Self> {
        let rest_url = Url::parse(&format!("{}://{}/v1/", protocol, host))?;

        let client = AsyncClient::builder()
            .danger_accept_invalid_certs(true)
            .build()?;

        let api = AsyncMarketstack {
            client,
            rest_url,
            auth,
        };

        // Ensure the API is working.
        api.auth.check_connection_async(&api).await?;

        Ok(api)
    }

    async fn send<T>(&self, req: reqwest::RequestBuilder) -> MarketstackResult<T>
    where
        T: DeserializeOwned,
    {
        let rsp = req.headers(HeaderMap::default()).send().await?;
        let status = rsp.status();
        if status.is_server_error() {
            return Err(MarketstackError::http(status));
        }

        serde_json::from_slice::<T>(&rsp.bytes().await?).map_err(MarketstackError::data_type::<T>)
    }

    async fn rest_async_simple(
        &self,
        mut request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, api::ApiError<<Self as api::RestClient>::Error>> {
        use futures_util::TryFutureExt;
        let call = || async {
            let http_request = request.body(body)?;
            let request = http_request.try_into()?;
            let rsp = self.client.execute(request).await?;

            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            for (key, value) in rsp.headers() {
                headers.insert(key, value.clone());
            }
            Ok(http_rsp.body(rsp.bytes().await?)?)
        };
        call().map_err(api::ApiError::client).await
    }
}
