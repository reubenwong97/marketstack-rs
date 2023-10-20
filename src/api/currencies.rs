//! Implementation of the `currencies` API endpoint.
//!
//! This endpoint is used to lookup the currencies supported by Marketstack.

use derive_builder::Builder;

use crate::api::paged::PaginationError;
use crate::api::{endpoint_prelude::*, ApiError};

/// Query for `currencies`.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct Currencies {
    /// Pagination limit for API request.
    #[builder(setter(name = "_limit"), default)]
    limit: Option<PageLimit>,
    /// Pagination offset value for API request.
    #[builder(default)]
    offset: Option<u64>,
}

impl Currencies {
    /// Create a builder for this endpoint.
    pub fn builder() -> CurrenciesBuilder {
        CurrenciesBuilder::default()
    }
}

impl CurrenciesBuilder {
    /// Limit the number of results returned.
    pub fn limit(&mut self, limit: u16) -> Result<&mut Self, ApiError<PaginationError>> {
        let new = self;
        new.limit = Some(Some(PageLimit::new(limit)?));
        Ok(new)
    }
}

impl<'a> Endpoint for Currencies {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "currencies".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push_opt("limit", self.limit.clone())
            .push_opt("offset", self.offset);

        params
    }
}

#[cfg(test)]
mod tests {

    use crate::api::currencies::Currencies;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn currencies_defaults_are_sufficient() {
        Currencies::builder().build().unwrap();
    }

    #[test]
    fn currencies_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("currencies")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Currencies::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn currencies_limit() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("currencies")
            .add_query_params(&[("limit", "50")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Currencies::builder().limit(50).unwrap().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn currencies_over_limit() {
        assert!(Currencies::builder().limit(9999).is_err());
    }

    #[test]
    fn currencies_offset() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("currencies")
            .add_query_params(&[("offset", "2")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Currencies::builder().offset(2).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
