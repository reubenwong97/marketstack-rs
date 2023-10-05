use std::any;
use std::convert::TryInto;
use std::fmt::{self, Debug};

use async_trait::async_trait;
use bytes::Bytes;
use http::{HeaderMap, Response as HttpResponse};
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
    // pub fn new<H, T>(host: T, token: T) -> MarketstackResult<Self>
    // where
    //     H: AsRef<str>,
    //     T: Into<String>,
    // {
    //     Self::new_impl()
    // }

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
        // api.auth.check_connection(&api)?;

        Ok(api)
    }
}

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
        #[source]
        source: http::Error,
    },
}
