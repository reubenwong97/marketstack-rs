//! Implemented endpoints for `intraday`, `intraday/latest` and `intraday/[date]`
//!
//! # Example
//!
//! ```rust,no_run
//! use marketstack::api::common::Interval;
//! use marketstack::api::{self, Query};
//! use marketstack::api::intraday::Intraday;
//! use marketstack::{Marketstack, IntradayData};
//!
//! // Create an insecure client.
//! let client = Marketstack::new_insecure("api.marketstack.com", "private-token").unwrap();
//!
//! // Create the `intraday` endpoint.
//! let endpoint = Intraday::builder()
//!     .symbol("AAPL")
//!     .interval(Interval::ThirtyMinutes)
//!     .build()
//!     .unwrap();
//!
//! // Call the endpoint. The return type decides how to represent the value.
//! let intraday_data: IntradayData = endpoint.query(&client).unwrap();
//!
//! // Data has been deserialized for you into `IntradayData`
//! assert_eq!(intraday_data.data.len(), 100);
//! assert!(intraday_data.data.iter().all(|intraday| intraday.symbol == "AAPL"));
//! ```
//!
//! Similar to the `eod` endpoint, `intraday` supports the same endpoint features.
//!
//! # Using Intraday Features
//!
//! ```rust,no_run
//! use chrono::NaiveDate;
//!
//! use marketstack::api::common::Interval;
//! use marketstack::api::{self, Query};
//! use marketstack::api::intraday::Intraday;
//! use marketstack::{Marketstack, IntradayData};
//!
//! let client = Marketstack::new_insecure("api.marketstack.com", "private-token").unwrap();
//!
//! // Create endpoint for `intraday/latest`.
//! let endpoint = Intraday::builder().symbol("AAPL").latest(true).build().unwrap();
//!
//! // OR create endpoint for `intraday/[date]`.
//! let endpoint = Intraday::builder()
//!     .symbol("AAPL")
//!     .date(NaiveDate::from_ymd(2019, 1, 1))
//!     .interval(Interval::ThirtyMinutes)
//!     .build()
//!     .unwrap();
//!
//! // Call the endpoint. The return type decides how to represent the value.
//! let intraday_data: IntradayData = endpoint.query(&client).unwrap();
//!
//! // Data has been deserialized for you into `IntradayData`
//! assert_eq!(intraday_data.data.len(), 14);  // 14 30-minute trading intervals in a day
//! ```

use std::collections::BTreeSet;

use chrono::NaiveDate;
use derive_builder::Builder;

use crate::api::common::{Interval, SortOrder};
use crate::api::paged::PaginationError;
use crate::api::{endpoint_prelude::*, ApiError};

/// Query for `intraday` endpoint
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option), build_fn(validate = "Self::validate"))]
pub struct Intraday<'a> {
    /// Search for eod for a symbol.
    #[builder(setter(name = "_symbols"), default)]
    symbols: BTreeSet<Cow<'a, str>>,
    /// Exchange to filer symbol by.
    #[builder(setter(into), default)]
    exchange: Option<Cow<'a, str>>,
    /// Preferred data interval.
    #[builder(default)]
    interval: Option<Interval>,
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
    /// Used when desired endpoint is `intraday/latest`
    #[builder(default)]
    latest: Option<bool>,
    /// Used when desired endpoint is `intraday/[date]`
    #[builder(default)]
    date: Option<NaiveDate>,
}

impl<'a> Intraday<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> IntradayBuilder<'a> {
        IntradayBuilder::default()
    }
}

impl<'a> IntradayBuilder<'a> {
    /// Search the given symbol.
    ///
    /// This provides sane defaults for the user to call symbol()
    /// on the builder without needing to wrap his symbol in a
    /// BTreeSet beforehand.
    pub fn symbol(&mut self, symbol: &'a str) -> &mut Self {
        self.symbols
            .get_or_insert_with(BTreeSet::new)
            .insert(symbol.into());
        self
    }

    /// Search the given symbols.
    pub fn symbols<I, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = V>,
        V: Into<Cow<'a, str>>,
    {
        self.symbols
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(|v| v.into()));
        self
    }

    /// Limit the number of results returned.
    pub fn limit(&mut self, limit: u16) -> Result<&mut Self, ApiError<PaginationError>> {
        let new = self;
        new.limit = Some(Some(PageLimit::new(limit)?));
        Ok(new)
    }

    /// Check that `Intraday` contains valid endpoint combinations.
    fn validate(&self) -> Result<(), String> {
        if self.date.is_some() && self.latest.is_some() {
            Err("Cannot use both `date` and `latest`".into())
        } else {
            Ok(())
        }
    }
}

impl<'a> Endpoint for Intraday<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        if self.latest.is_some() {
            "intraday/latest".into()
        } else if self.date.is_some() {
            // Panics on invalid date -> irrecoverable and illegal to proceed
            format!("intraday/{}", self.date.unwrap()).into()
        } else {
            "intraday".into()
        }
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .extend(self.symbols.iter().map(|value| ("symbols", value)))
            .push_opt("exchange", self.exchange.as_ref())
            .push_opt("interval", self.interval.clone())
            .push_opt("sort", self.sort)
            .push_opt("date_from", self.date_from)
            .push_opt("date_to", self.date_to)
            .push_opt("limit", self.limit.clone())
            .push_opt("offset", self.offset);

        params
    }
}

#[cfg(test)]
mod tests {

    use chrono::NaiveDate;

    use crate::api::common::{Interval, SortOrder};
    use crate::api::intraday::Intraday;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn intraday_defaults_are_sufficient() {
        Intraday::builder().build().unwrap();
    }

    #[test]
    fn intraday_endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("intraday").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Intraday::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn intraday_symbol() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("intraday")
            .add_query_params(&[("symbols", "AAPL")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Intraday::builder().symbol("AAPL").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn intraday_symbols() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("intraday")
            .add_query_params(&[("symbols", "AAPL"), ("symbols", "MSFT")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Intraday::builder()
            .symbols(["AAPL", "MSFT"].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn intraday_exchange() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("intraday")
            .add_query_params(&[("exchange", "NYSE")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Intraday::builder().exchange("NYSE").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn intraday_interval() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("intraday")
            .add_query_params(&[("interval", "5min")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Intraday::builder()
            .interval(Interval::FiveMinutes)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn intraday_sort() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("intraday")
            .add_query_params(&[("sort", "ASC")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Intraday::builder()
            .sort(SortOrder::Ascending)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn intraday_date_from_and_to() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("intraday")
            .add_query_params(&[("date_from", "2019-01-01"), ("date_to", "2019-01-02")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Intraday::builder()
            .date_from(NaiveDate::from_ymd_opt(2019, 1, 1).unwrap())
            .date_to(NaiveDate::from_ymd_opt(2019, 1, 2).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn intraday_limit_and_offset() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("intraday")
            .add_query_params(&[("limit", "5"), ("offset", "3")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Intraday::builder()
            .limit(5)
            .unwrap()
            .offset(3)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn intraday_over_limit() {
        assert!(Intraday::builder().limit(5000).is_err());
    }

    #[test]
    fn intraday_latest_defaults_are_sufficient() {
        Intraday::builder().latest(true).build().unwrap();
    }

    #[test]
    fn intraday_latest_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("intraday/latest")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Intraday::builder().latest(true).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn intraday_date_defaults_are_sufficient() {
        Intraday::builder()
            .date(NaiveDate::from_ymd_opt(2019, 1, 1).unwrap())
            .build()
            .unwrap();
    }

    #[test]
    fn intraday_date_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("intraday/2019-01-01")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Intraday::builder()
            .date(NaiveDate::from_ymd_opt(2019, 1, 1).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
