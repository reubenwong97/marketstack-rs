//! Implementation of the `dividends` API endpoint.

use std::collections::BTreeSet;

use chrono::NaiveDate;
use derive_builder::Builder;

use crate::api::common::SortOrder;
use crate::api::paged::PaginationError;
use crate::api::{endpoint_prelude::*, ApiError};

/// Query for `dividends`.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Dividends<'a> {
    /// Search for `dividends` for a symbol.
    #[builder(setter(name = "_symbols"), default)]
    symbols: BTreeSet<Cow<'a, str>>,
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
}

impl<'a> Dividends<'a> {
    /// Create a builder for this endpoint.
    pub fn builder() -> DividendsBuilder<'a> {
        DividendsBuilder::default()
    }
}

impl<'a> DividendsBuilder<'a> {
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
}

impl<'a> Endpoint for Dividends<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "dividends".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .extend(self.symbols.iter().map(|value| ("symbols", value)))
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
    use crate::api::dividends::Dividends;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn dividends_defaults_are_sufficient() {
        Dividends::builder().build().unwrap();
    }

    #[test]
    fn dividends_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dividends")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Dividends::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn dividends_symbol() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dividends")
            .add_query_params(&[("symbols", "AAPL")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Dividends::builder().symbol("AAPL").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn dividends_symbols() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dividends")
            .add_query_params(&[("symbols", "AAPL"), ("symbols", "GOOG")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Dividends::builder()
            .symbol("AAPL")
            .symbols(["AAPL", "GOOG"].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn dividends_sort() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dividends")
            .add_query_params(&[("sort", "ASC")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Dividends::builder()
            .sort(SortOrder::Ascending)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn dividends_date_from() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dividends")
            .add_query_params(&[("date_from", "2020-01-01")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Dividends::builder()
            .date_from(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn dividends_date_to() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dividends")
            .add_query_params(&[("date_to", "2020-01-01")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Dividends::builder()
            .date_to(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn dividends_limit() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dividends")
            .add_query_params(&[("limit", "50")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Dividends::builder().limit(50).unwrap().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn dividends_over_limit() {
        assert!(Dividends::builder().limit(9999).is_err());
    }

    #[test]
    fn dividends_offset() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dividends")
            .add_query_params(&[("offset", "2")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Dividends::builder().offset(2).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
