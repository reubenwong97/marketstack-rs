//! Implemented endpoints for `intraday`, `intraday/latest` and `intraday/[date]`

use std::collections::BTreeSet;

use chrono::NaiveDate;
use derive_builder::Builder;

use crate::api::common::SortOrder;
use crate::api::{endpoint_prelude::*, ApiError};

/// Query for `intraday` endpoint
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct Intraday<'a> {
    /// Search for eod for a symbol.
    #[builder(setter(name = "_symbols"), default)]
    symbols: BTreeSet<Cow<'a, str>>,
    /// Exchange to filer symbol by.
    #[builder(setter(into), default)]
    exchange: Option<Cow<'a, str>>,
    /// The sort order for the return results.
    #[builder(default)]
    sort: Option<SortOrder>,
    /// Date to query EOD data from.
    #[builder(default)]
    date_from: Option<NaiveDate>,
    /// Date to query EOD date to.
    #[builder(default)]
    date_to: Option<NaiveDate>,
    /// Pagination limit for API request.
    #[builder(setter(name = "_limit"), default)]
    limit: Option<PageLimit>,
    /// Pagination offset value for API request.
    #[builder(default)]
    offset: Option<u64>,
    /// Used when desired endpoint is `eod/latest`
    #[builder(default)]
    latest: Option<bool>,
    /// Used when desired endpoint is `eod/[date]`
    #[builder(default)]
    date: Option<NaiveDate>,
}
