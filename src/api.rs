#![warn(missing_docs)]

//! API endpoint structures.
//!
//! The types in this module are meant to aid in constructing the appropriate calls using type-safe
//! Rust idioms.
//!
//! All endpoints use the builder pattern and have their members as private so that there are no
//! API implications of adding new members for additional query parameters in future GitLab
//! releases.
//!
//! # Example
//!
//! ```rust,no_run
//! use serde::{Deserialize, Serialize};
//! use marketstack::Marketstack;
//! use marketstack::api::{self, Query};
//! use marketstack::api::eod;
//! use marketstack::{PaginationInfo, EodDataItem};
//!
//! // The return type of an `EodData`. Note that Marketstack may contain more information, but you can
//! // define your structure to only fetch what is needed.
//! #[derive(Serialize, Deserialize, Debug, Clone)]
//! pub struct EodData {
//!     pub pagination: PaginationInfo,
//!     pub data: Vec<EodDataItem>,
//! }
//!
//! // Create the client.
//! let client = Marketstack::new("api.marketstack.com", "private-token").unwrap();
//!
//! // OR create an insecure token (if on the Free plan).
//! let client = Marketstack::new_insecure("api.marketstack.com", "private-token").unwrap();
//!
//! // Create a simple endpoint. This one gets the "eod" for the AAPL symbol.
//! let endpoint = eod::Eod::builder().symbol("AAPL").build().unwrap();
//! // Call the endpoint. The return type decides how to represent the value.
//! let eod_date: EodData = endpoint.query(&client).unwrap();
//!
//! // Some endpoints support pagination. Since Marketstack does pagination through query
//! // params, we simply specify them in the endpoint builder.
//! // Note that there are limits defined, and therefore, limit(5) is fallible and returns
//! // a Result.
//! let pageable_endpoint = eod::Eod::builder().symbol("AAPL").limit(5).unwrap().build().unwrap();
//! ```

mod client;
mod endpoint;
mod error;
mod ignore;
mod params;
mod query;
mod raw;

pub mod endpoint_prelude;

pub mod common;
pub mod currencies;
pub mod dividends;
pub mod eod;
pub mod paged;
pub mod splits;

pub use self::client::AsyncClient;
pub use self::client::Client;
pub use self::client::RestClient;

pub use self::endpoint::Endpoint;

pub use self::error::ApiError;
pub use self::error::BodyError;

pub use self::ignore::ignore;
pub use self::ignore::Ignore;

pub use self::params::FormParams;
pub use self::params::ParamValue;
pub use self::params::QueryParams;

pub use self::query::AsyncQuery;
pub use self::query::Query;

pub use self::raw::raw;
pub use self::raw::Raw;

pub use self::paged::PageLimit;
