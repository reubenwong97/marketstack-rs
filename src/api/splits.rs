//! Implemented endpoints for `splits`

use std::collections::BTreeSet;

use chrono::NaiveDate;
use derive_builder::Builder;

use crate::api::common::SortOrder;
use crate::api::paged::PaginationError;
use crate::api::{endpoint_prelude::*, ApiError};

/// Query for `splits`.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Splits<'a> {
    /// Search for `splits` for a symbol.
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

impl<'a> Splits<'a> {
    /// Create a bulder for this endpoint.
    pub fn builder() -> SplitsBuilder<'a> {
        SplitsBuilder::default()
    }
}

impl<'a> SplitsBuilder<'a> {
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

impl<'a> Endpoint for Splits<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "splits".into()
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

    use crate::api::splits::Splits;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn splits_defaults_are_sufficient() {
        Splits::builder().build().unwrap();
    }

    #[test]
    fn splits_endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("splits").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Splits::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
