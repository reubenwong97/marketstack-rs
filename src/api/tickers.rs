//! Implemented for `tickers` and associated endpoints.

use std::borrow::Cow;

use derive_builder::Builder;

use crate::api::eod::Eod;
use crate::api::paged::PaginationError;
use crate::api::{endpoint_prelude::*, ApiError};

/// Base for `tickers`.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Tickers<'a> {
    /// Ticker symbol.
    #[builder(setter(into), default)]
    ticker: Option<Cow<'a, str>>,
    /// To filter your results based on a specific stock exchange, use this parameter to specify the MIC identification of a stock exchange. Example: `XNAS`
    #[builder(setter(into), default)]
    exchange: Option<Cow<'a, str>>,
    /// Use this parameter to search stock tickers by name or ticker symbol.
    #[builder(setter(into), default)]
    search: Option<Cow<'a, str>>,
    /// Pagination limit for API request.
    #[builder(setter(name = "_limit"), default)]
    limit: Option<PageLimit>,
    /// Pagination offset value for API request.
    #[builder(default)]
    offset: Option<u64>,
    /// `Eod` struct being built, and held by the `Tickers` struct.
    #[builder(setter(into), default)]
    eod: Option<Eod<'a>>,
}

impl<'a> Tickers<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> TickersBuilder<'a> {
        TickersBuilder::default()
    }
}

impl<'a> Endpoint for Tickers<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        let mut endpoint = "tickers".to_owned();
        if let Some(ticker) = &self.ticker {
            endpoint.push_str(&format!("/{}", ticker));

            if let Some(eod) = &self.eod {
                endpoint.push_str(&format!("/{}", eod.endpoint().as_ref()));
            }
        }

        endpoint.into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        // NOTE: Not the most ergonomic way I want to go about this, but its okay for now since
        // only one "extension" endpoint like `eod` or `splits` can be active per `tickers`
        // endpoint query to Marketstack.
        if let Some(eod) = &self.eod {
            params = eod.parameters().clone();
        }

        // Push params from the `tickers` endpoint.
        params
            .push_opt("exchange", self.exchange.as_ref())
            .push_opt("search", self.search.as_ref())
            .push_opt("limit", self.limit.clone())
            .push_opt("offset", self.offset);

        params
    }
}

impl<'a> TickersBuilder<'a> {
    /// Limit the number of results returned.
    pub fn limit(&mut self, limit: u16) -> Result<&mut Self, ApiError<PaginationError>> {
        let new = self;
        new.limit = Some(Some(PageLimit::new(limit)?));
        Ok(new)
    }
}

#[cfg(test)]
mod tests {

    use chrono::NaiveDate;

    use crate::api::common::SortOrder;
    use crate::api::eod::Eod;
    use crate::api::tickers::Tickers;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn tickers_defaults_are_sufficient() {
        Tickers::builder().build().unwrap();
    }

    #[test]
    fn tickers_ticker() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("tickers/AAPL")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Tickers::builder().ticker("AAPL").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn tickers_eod_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("tickers/AAPL/eod")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Tickers::builder()
            .ticker("AAPL")
            .eod(Eod::builder().build().unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn tickers_eod_latest_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("tickers/AAPL/eod/latest")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Tickers::builder()
            .ticker("AAPL")
            .eod(Eod::builder().latest(true).build().unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn tickers_eod_date_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("tickers/AAPL/eod/2023-09-27")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Tickers::builder()
            .ticker("AAPL")
            .eod(
                Eod::builder()
                    .date(NaiveDate::from_ymd_opt(2023, 9, 27).unwrap())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn tickers_eod_params() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("tickers/AAPL/eod")
            .add_query_params(&[("sort", "ASC")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Tickers::builder()
            .ticker("AAPL")
            .eod(Eod::builder().sort(SortOrder::Ascending).build().unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
