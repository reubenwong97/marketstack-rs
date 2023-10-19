use std::collections::BTreeSet;

use derive_builder::Builder;

use crate::api::common::SortOrder;
use crate::api::paged::PaginationError;
use crate::api::{endpoint_prelude::*, ApiError};

/// Query for eod.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct EodLatest<'a> {
    /// Search for eod for a symbol.
    #[builder(setter(name = "_symbols"), default)]
    symbols: BTreeSet<Cow<'a, str>>,
    /// Exchange to filer symbol by.
    #[builder(setter(into), default)]
    exchange: Option<Cow<'a, str>>,
    /// The sort order for the return results.
    #[builder(default)]
    sort: Option<SortOrder>,
    /// Pagination limit for API request.
    #[builder(setter(name = "_limit"), default)]
    limit: Option<PageLimit>,
    /// Pagination offset value for API request.
    #[builder(default)]
    offset: Option<u64>,
}

impl<'a> EodLatest<'a> {
    pub fn builder() -> EodLatestBuilder<'a> {
        EodLatestBuilder::default()
    }
}

impl<'a> EodLatestBuilder<'a> {
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

    pub fn limit(&mut self, limit: u16) -> Result<&mut Self, ApiError<PaginationError>> {
        let new = self;
        new.limit = Some(Some(PageLimit::new(limit)?));
        Ok(new)
    }
}

impl<'a> Endpoint for EodLatest<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "eod/latest".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .extend(self.symbols.iter().map(|value| ("symbols", value)))
            .push_opt("exchange", self.exchange.as_ref())
            .push_opt("sort", self.sort)
            .push_opt("limit", self.limit.clone())
            .push_opt("offset", self.offset);

        params
    }
}

#[cfg(test)]
mod tests {

    use crate::api::common::SortOrder;
    use crate::api::eod::EodLatest;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn eod_defaults_are_sufficient() {
        EodLatest::builder().build().unwrap();
    }

    #[test]
    fn eod_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod/latest")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EodLatest::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_symbol() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod/latest")
            .add_query_params(&[("symbols", "AAPL")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EodLatest::builder().symbol("AAPL").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_symbols() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod/latest")
            .add_query_params(&[("symbols", "AAPL"), ("symbols", "GOOG")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EodLatest::builder()
            .symbol("AAPL")
            .symbols(["AAPL", "GOOG"].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_exchange() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod/latest")
            .add_query_params(&[("exchange", "NYSE")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EodLatest::builder().exchange("NYSE").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_sort() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod/latest")
            .add_query_params(&[("sort", "ASC")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EodLatest::builder()
            .sort(SortOrder::Ascending)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_limit() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod/latest")
            .add_query_params(&[("limit", "50")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EodLatest::builder().limit(50).unwrap().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn eod_over_limit() {
        assert!(EodLatest::builder().limit(9999).is_err());
    }

    #[test]
    fn eod_offset() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("eod/latest")
            .add_query_params(&[("offset", "2")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EodLatest::builder().offset(2).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
