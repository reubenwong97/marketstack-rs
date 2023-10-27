//! Implemented for `tickers` and associated endpoints.

use std::borrow::Cow;

use derive_builder::Builder;

use crate::api::eod::Eod;

/// Base for `tickers`.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Tickers<'a> {
    /// Ticker symbol.
    #[builder(setter(into), default)]
    ticker: Option<Cow<'a, str>>,
    /// `Eod` struct being built, and held by the `Tickers` struct.
    #[builder(setter(skip))]
    eod_struct: Option<Eod<'a>>,
}

impl<'a> Tickers<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> TickersBuilder<'a> {
        TickersBuilder::default()
    }
}
