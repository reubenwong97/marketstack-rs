//! Endpoint prelude
//!
//! This module re-exports all of the types needed for endpoints to implement the
//! [`Endpoint`](../trait.Endpoint.html) trait.

pub use std::borrow::Cow;

pub use http::Method;

pub use crate::api::paged::PageLimit;
pub use crate::api::BodyError;
pub use crate::api::Client;
pub use crate::api::Endpoint;
pub use crate::api::FormParams;
pub use crate::api::QueryParams;
