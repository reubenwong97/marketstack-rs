//! Implemented endpoints for `eod`, `eod/latest `and `eod/[date]`.
//!
//! # Example
//!
//! ```rust,no_run
//! use marketstack::api::{self, Query};
//! use marketstack::api::eod::Eod;
//! use marketstack::{Marketstack, EodData};
//!
//! // Create an insecure client.
//! let client = Marketstack::new_insecure("api.marketstack.com", "private-token").unwrap();
//!
//! // Create the eod endpoint.
//! let endpoint = Eod::builder().symbol("AAPL").build().unwrap();
//!
//! // Call the endpoint. The return type decides how to represent the value.
//! let eod_data: EodData = endpoint.query(&client).unwrap();
//!
//! // Data has been deserialized for you into `EodData`.
//! assert_eq!(eod_data.data.len(), 100);
//! assert_eq!(eod_data.pagination.limit, 100);
//! assert!(eod_data.data.iter().all(|eod| eod.symbol == "AAPL"));
//! ```
//!
//! Beyond the simple `eod` endpoint, the Marketstack API also implements
//! "endpoint features", which extend the endpoint for different behaviour.
//! The two available ones take the form of `eod/latest` and `eod/[date]`.
//!
//! # Using Eod Features
//!
//! ```rust,no_run
//! use chrono::NaiveDate;
//!
//! use marketstack::api::{self, Query};
//! use marketstack::api::eod::Eod;
//! use marketstack::{Marketstack, EodData};
//!
//! let client = Marketstack::new_insecure("api.marketstack.com", "private-token").unwrap();
//!
//! // Create endpoint for `eod/latest`.
//! let endpoint = Eod::builder().latest(true).build().unwrap();
//!
//! // OR create endpoint for `eod/[date]`
//! let endpoint = Eod::builder().date(NaiveDate::from_ymd_opt(2022, 1, 4).unwrap()).build().unwrap();
//!
//! // Call the endpoint.
//! let eod_data: EodData = endpoint.query(&client).unwrap();
//!
//! assert_eq!(eod_data.data.len(), 1);
//! ```
//!
//! Note that `eod/latest` and `eod/[date]` cannot be used together.
//!
//! ```rust,no_run
//! use chrono::NaiveDate;
//!
//! use marketstack::api::eod::Eod;
//!
//! let endpoint = Eod::builder().latest(true).date(NaiveDate::from_ymd(2022, 1, 4)).build();
//!
//! assert!(endpoint.is_err());
//! ```

use std::collections::BTreeSet;

use chrono::NaiveDate;
use derive_builder::Builder;

use crate::api::common::SortOrder;
use crate::api::paged::PaginationError;
use crate::api::{endpoint_prelude::*, ApiError};

/// Query for `eod`.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option), build_fn(validate = "Self::validate"))]
pub struct Eod<'a> {
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

impl<'a> Eod<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> EodBuilder<'a> {
        EodBuilder::default()
    }
}

impl<'a> EodBuilder<'a> {
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

    /// Check that `Eod` contains valid endpoint combinations.
    fn validate(&self) -> Result<(), String> {
        if self.date.is_some() && self.latest.is_some() {
            Err("Cannot use both `date` and `latest`".into())
        } else {
            Ok(())
        }
    }
}

impl<'a> Endpoint for Eod<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        if self.latest.is_some() {
            "eod/latest".into()
        } else if self.date.is_some() {
            // Panics on invalid date -> irrecoverable and illegal to proceed
            format!("eod/{}", self.date.unwrap()).into()
        } else {
            "eod".into()
        }
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .extend(self.symbols.iter().map(|value| ("symbols", value)))
            .push_opt("exchange", self.exchange.as_ref())
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

    use crate::api::common::SortOrder;
    use crate::api::eod::Eod;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn eod_defaults_are_sufficient() {
        Eod::builder().build().unwrap();
    }

    #[test]
    fn eod_endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("eod").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Eod::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_symbol() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod")
            .add_query_params(&[("symbols", "AAPL")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Eod::builder().symbol("AAPL").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_symbols() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod")
            .add_query_params(&[("symbols", "AAPL"), ("symbols", "GOOG")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Eod::builder()
            .symbol("AAPL")
            .symbols(["AAPL", "GOOG"].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_exchange() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod")
            .add_query_params(&[("exchange", "NYSE")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Eod::builder().exchange("NYSE").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_sort() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod")
            .add_query_params(&[("sort", "ASC")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Eod::builder().sort(SortOrder::Ascending).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_date_from() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod")
            .add_query_params(&[("date_from", "2020-01-01")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Eod::builder()
            .date_from(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_date_to() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod")
            .add_query_params(&[("date_to", "2020-01-01")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Eod::builder()
            .date_to(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_limit() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod")
            .add_query_params(&[("limit", "50")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Eod::builder().limit(50).unwrap().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_over_limit() {
        assert!(Eod::builder().limit(9999).is_err());
    }

    #[test]
    fn eod_offset() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod")
            .add_query_params(&[("offset", "2")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Eod::builder().offset(2).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_latest_defaults_are_sufficient() {
        Eod::builder().latest(true).build().unwrap();
    }

    #[test]
    fn eod_latest_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod/latest")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Eod::builder().latest(true).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_latest_symbol() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod/latest")
            .add_query_params(&[("symbols", "AAPL")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Eod::builder().latest(true).symbol("AAPL").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_latest_symbols() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod/latest")
            .add_query_params(&[("symbols", "AAPL"), ("symbols", "GOOG")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Eod::builder()
            .latest(true)
            .symbol("AAPL")
            .symbols(["AAPL", "GOOG"].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_latest_exchange() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod/latest")
            .add_query_params(&[("exchange", "NYSE")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Eod::builder()
            .latest(true)
            .exchange("NYSE")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_latest_sort() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod/latest")
            .add_query_params(&[("sort", "ASC")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Eod::builder()
            .latest(true)
            .sort(SortOrder::Ascending)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_latest_limit() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod/latest")
            .add_query_params(&[("limit", "50")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Eod::builder()
            .latest(true)
            .limit(50)
            .unwrap()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_latest_over_limit() {
        assert!(Eod::builder().latest(true).limit(9999).is_err());
    }

    #[test]
    fn eod_latest_offset() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod/latest")
            .add_query_params(&[("offset", "2")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Eod::builder().latest(true).offset(2).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_date_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod/2022-01-01")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Eod::builder()
            .date(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_date_symbol() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod/2022-01-01")
            .add_query_params(&[("symbols", "AAPL")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Eod::builder()
            .symbol("AAPL")
            .date(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_date_symbols() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod/2022-01-01")
            .add_query_params(&[("symbols", "AAPL"), ("symbols", "GOOG")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Eod::builder()
            .symbol("AAPL")
            .date(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap())
            .symbols(["AAPL", "GOOG"].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_date_exchange() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod/2022-01-01")
            .add_query_params(&[("exchange", "NYSE")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Eod::builder()
            .exchange("NYSE")
            .date(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_date_sort() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod/2022-01-01")
            .add_query_params(&[("sort", "ASC")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Eod::builder()
            .sort(SortOrder::Ascending)
            .date(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_date_limit() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod/2022-01-01")
            .add_query_params(&[("limit", "50")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Eod::builder()
            .date(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap())
            .limit(50)
            .unwrap()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_date_over_limit() {
        assert!(Eod::builder()
            .date(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap())
            .limit(9999)
            .is_err());
    }

    #[test]
    fn eod_date_offset() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod/2022-01-01")
            .add_query_params(&[("offset", "2")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Eod::builder()
            .date(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap())
            .offset(2)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_date_latest_mutually_exclusive() {
        let endpoint = Eod::builder()
            .latest(true)
            .date(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap())
            .build();

        assert!(endpoint.is_err());
        assert!(endpoint
            .err()
            .unwrap()
            .to_string()
            .contains("Cannot use both"));
    }
}
