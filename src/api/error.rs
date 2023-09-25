// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::any;
use std::error::Error;

use thiserror::Error;

/// Errors which may occur when creating form data.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BodyError {
    /// Body data could not be serialized from form parameters.
    #[error("failed to URL encode form parameters: {}", source)]
    UrlEncoded {
        /// The source of the error.
        #[from]
        source: serde_urlencoded::ser::Error,
    },
}

/// Errors which may occur when using API endpoints.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    /// The client encountered an error.
    #[error("client error: {}", source)]
    Client {
        /// The client error.
        source: E,
    },
    /// The URL failed to parse.
    #[error("failed to parse URL: {}", source)]
    UrlParse {
        /// The source of the error.
        #[from]
        source: url::ParseError,
    },
    /// Body data could not be created.
    #[error("failed to create request body: {}", source)]
    Body {
        /// The source of the error.
        #[from]
        source: BodyError,
    },
    /// JSON deserialization from Marketstack failed.
    #[error("could not parse JSON response: {}", source)]
    Json {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },
    /// Marketstack returned an error message.
    #[error("marketstack server error: {}", msg)]
    Marketstack {
        /// The error message from Marketstack.
        msg: String,
    },
    /// Marketstack returned an error without JSON information.
    #[error("marketstack internal server error: {}", status)]
    MarketstackService {
        /// The status code for the return.
        status: http::StatusCode,
        /// The error data from Marketstack.
        data: Vec<u8>,
    },
    /// Marketstack returned an error object.
    #[error("marketstack server error: {:?}", obj)]
    MarketstackObject {
        /// The error object from Marketstack.
        obj: serde_json::Value,
    },
    /// Marketstack returned an HTTP error with JSON we did not recognize.
    #[error("marketstack server error: {:?}", obj)]
    MarketstackUnrecognized {
        /// The full object from Marketstack.
        obj: serde_json::Value,
    },
    /// Failed to parse an expected data type from JSON.
    #[error("could not parse {} data from JSON: {}", typename, source)]
    DataType {
        /// The source of the error.
        source: serde_json::Error,
        /// The name of the type that could not be deserialized.
        typename: &'static str,
    },
    // TODO: implement pagination error
}

impl<E> ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    /// Create an API error in a client error.
    pub fn client(source: E) -> Self {
        ApiError::Client { source }
    }

    /// Wrap a client error in another wrapper.
    pub fn map_client<F, W>(self, f: F) -> ApiError<W>
    where
        F: FnOnce(E) -> W,
        W: Error + Send + Sync + 'static,
    {
        match self {
            Self::Client { source } => ApiError::client(f(source)),
            Self::UrlParse { source } => ApiError::UrlParse { source },
            Self::Body { source } => ApiError::Body { source },
            Self::Json { source } => ApiError::Json { source },
            Self::Marketstack { msg } => ApiError::Marketstack { msg },
            Self::MarketstackService { status, data } => {
                ApiError::MarketstackService { status, data }
            }
            Self::MarketstackObject { obj } => ApiError::MarketstackObject { obj },
            Self::MarketstackUnrecognized { obj } => ApiError::MarketstackUnrecognized { obj },
            Self::DataType { source, typename } => ApiError::DataType { source, typename },
        }
    }

    pub(crate) fn server_error(status: http::StatusCode, body: &bytes::Bytes) -> Self {
        Self::MarketstackService {
            status,
            data: body.into_iter().copied().collect(),
        }
    }

    pub(crate) fn from_marketstack(value: serde_json::Value) -> Self {
        let error_value = value
            .pointer("/message")
            .or_else(|| value.pointer("/error"));

        if let Some(error_value) = error_value {
            if let Some(msg) = error_value.as_str() {
                ApiError::Marketstack { msg: msg.into() }
            } else {
                ApiError::MarketstackObject {
                    obj: error_value.clone(),
                }
            }
        } else {
            ApiError::MarketstackUnrecognized { obj: value }
        }
    }

    pub(crate) fn data_type<T>(source: serde_json::Error) -> Self {
        ApiError::DataType {
            source,
            typename: any::type_name::<T>(),
        }
    }
}
