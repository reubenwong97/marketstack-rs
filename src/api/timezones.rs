//! Implementation of the `timezones` API endpoint.
//!
//! This endpoint is used to lookup the timezones supported by Marketstack.

use derive_builder::Builder;

use crate::api::paged::PaginationError;
use crate::api::{endpoint_prelude::*, ApiError};

/// Query for `timezones`.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct Timezones {
    /// Pagination limit for API request.
    #[builder(setter(name = "_limit"), default)]
    limit: Option<PageLimit>,
    /// Pagination offset value for API request.
    #[builder(default)]
    offset: Option<u64>,
}

impl Timezones {
    /// Create a builder for this endpoint.
    pub fn builder() -> TimezonesBuilder {
        TimezonesBuilder::default()
    }
}

impl TimezonesBuilder {
    /// Limit the number of results returned.
    pub fn limit(&mut self, limit: u16) -> Result<&mut Self, ApiError<PaginationError>> {
        let new = self;
        new.limit = Some(Some(PageLimit::new(limit)?));
        Ok(new)
    }
}

impl Endpoint for Timezones {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "timezones".into()
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

    use crate::api::timezones::Timezones;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn timezones_defaults_are_sufficient() {
        Timezones::builder().build().unwrap();
    }

    #[test]
    fn timezones_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("timezones")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Timezones::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn timezones_limit() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("timezones")
            .add_query_params(&[("limit", "50")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Timezones::builder().limit(50).unwrap().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn timezones_over_limit() {
        assert!(Timezones::builder().limit(9999).is_err());
    }

    #[test]
    fn timezones_offset() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("timezones")
            .add_query_params(&[("offset", "2")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Timezones::builder().offset(2).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
