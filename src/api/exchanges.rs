//! Implemented for `exchanges` and associated endpoints.

use std::borrow::Cow;

use derive_builder::Builder;

use crate::api::paged::PaginationError;
use crate::api::{endpoint_prelude::*, ApiError};

/// Base for `exchanges`.
#[derive(Debug, Builder, Clone)]
pub struct Exchanges<'a> {
    /// Search stock exchanges by name or MIC.
    #[builder(setter(into), default)]
    search: Option<Cow<'a, str>>,
    /// Pagination limit for API request.
    #[builder(setter(name = "_limit"), default)]
    limit: Option<PageLimit>,
    /// Pagination offset value for API request.
    #[builder(default)]
    offset: Option<u64>,
}

impl<'a> Exchanges<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ExchangesBuilder<'a> {
        ExchangesBuilder::default()
    }
}
