// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

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
