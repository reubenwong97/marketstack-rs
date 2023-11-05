//! Implemented for `exchanges` and associated endpoints.
//!
//! The `exchanges` endpoint is an interesting one as it adopts features and
//! params from other endpints like `eod`. Therefore, implementation for this module
//! re-uses the logic by nesting `eod`'s builder struct within `exchanges`'s builder.
//! I will provide an example below.
//!
//! # Nesting `eod`'s builder struct within `exchanges`'s builder
//!
//! ```rust,no_run
//! use chrono::NaiveDate;
//!
//! use marketstack::api::common::SortOrder;
//! use marketstack::api::{self, Query};
//! use marketstack::api::eod::Eod;
//! use marketstack::api::exchanges::Exchanges;
//! use marketstack::{Marketstack, ExchangesEodData};
//!
//! // Create an insecure client.
//! let client = Marketstack::new_insecure("api.marketstack.com", "private-token").unwrap();
//!
//! // Create the `exchanges/[mic]/eod` endpoint.
//! let endpoint = Exchanges::builder()
//!     .mic("XNAS")
//!     .eod(Eod::builder()
//!         .limit(5)
//!         .unwrap()
//!         .date_from(NaiveDate::from_ymd_opt(2023, 5, 5).unwrap())
//!         .date_to(NaiveDate::from_ymd_opt(2023, 10, 5).unwrap())
//!         .sort(SortOrder::Ascending)
//!         .symbols(["AAPL", "TSLA"].iter().copied())
//!         .build()
//!         .unwrap())
//!     .build()
//!     .unwrap();
//!
//! // Query the endpoint.
//! let exchanges_eod_result: ExchangesEodData = endpoint.query(&client).unwrap();
//! ```
//!
//! The interesting thing is about re-using the `Eod` builder is that it allows
//! all its features to be present even within the `exchanges` endpoint. For example,
//! we can use `exchanges/[mic]/eod/latest` by doing the following:
//!
//! ```rust,no_run
//! use marketstack::api::{self, Query};
//! use marketstack::api::exchanges::Exchanges;
//! use marketstack::{Marketstack, ExchangesEodData};
//! use marketstack::api::eod::Eod;
//!
//! // Create an insecure client.
//! let client = Marketstack::new_insecure("api.marketstack.com", "private-token").unwrap();
//!
//! // Create the `exchanges/[mic]/eod/latest` endpoint.
//! let endpoint = Exchanges::builder()
//!     .mic("XNAS")
//!     .eod(Eod::builder().latest(true).build().unwrap())
//!     .build()
//!     .unwrap();
//!
//! // Query the endpoint.
//! let exchanges_eod_latest_result: ExchangesEodData = endpoint.query(&client).unwrap();
//! ```

use std::borrow::Cow;

use derive_builder::Builder;

use crate::api::eod::Eod;
use crate::api::paged::PaginationError;
use crate::api::{endpoint_prelude::*, ApiError};

/// Base for `exchanges`.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option), build_fn(validate = "Self::validate"))]
pub struct Exchanges<'a> {
    /// Obtain information about a specific stock exchange by attaching its MIC
    /// identification to your API request URL, e.g. `/exchanges/XNAS`.
    #[builder(setter(into), default)]
    mic: Option<Cow<'a, str>>,
    /// Obtain all available tickers for a specific exchange by attaching the
    /// exchange MIC as well as `/tickers`, e.g. `/exchanges/XNAS/tickers`.
    #[builder(setter(into), default)]
    tickers: Option<Cow<'a, str>>,
    /// `Eod` struct being built, and held by the `Exchanges` struct.
    /// Results in the `/exchanges/[mic]/eod` endpoint.
    #[builder(setter(into), default)]
    eod: Option<Eod<'a>>,
    /// Search stock exchanges by name or MIC.
    #[builder(setter(into), default)]
    search: Option<Cow<'a, str>>,
    /// Pagination limit for API request.
    #[builder(setter(name = "_limit"), default)]
    limit: Option<PageLimit>,
    /// Pagination offset value for API request.
    #[builder(default)]
    offset: Option<u64>,
}

impl<'a> Exchanges<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ExchangesBuilder<'a> {
        ExchangesBuilder::default()
    }
}

impl<'a> Endpoint for Exchanges<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        let mut endpoint = "exchanges".to_owned();
        if let Some(mic) = &self.mic {
            endpoint.push_str(&format!("/{}", mic));

            // NOTE: validator will ensure only one can be active.
            if let Some(tickers) = &self.tickers {
                endpoint.push_str(&format!("/{}", tickers));
            }
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

        params
            .push_opt("search", self.search.as_ref())
            .push_opt("limit", self.limit.clone())
            .push_opt("offset", self.offset);

        params
    }
}

impl<'a> ExchangesBuilder<'a> {
    /// Limit the number of results returned.
    pub fn limit(&mut self, limit: u16) -> Result<&mut Self, ApiError<PaginationError>> {
        let new = self;
        new.limit = Some(Some(PageLimit::new(limit)?));
        Ok(new)
    }

    /// Check that `Exchanges` contains valid endpoint combinations
    fn validate(&self) -> Result<(), String> {
        let active_fields = [self.tickers.is_some(), self.eod.is_some()];
        let count = active_fields.iter().filter(|x| **x).count();

        if count > 1 {
            Err("Invalid combinations of `eod`, `tickers` or `intraday`".into())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {

    use chrono::NaiveDate;

    use crate::api::eod::Eod;
    use crate::api::exchanges::Exchanges;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn exchanges_defaults_are_sufficient() {
        Exchanges::builder().build().unwrap();
    }

    #[test]
    fn exchanges() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("exchanges")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Exchanges::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn exchanges_mic() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("exchanges/XNAS")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Exchanges::builder().mic("XNAS").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn exchanges_mic_tickers() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("exchanges/XNAS/tickers")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Exchanges::builder()
            .mic("XNAS")
            .tickers("tickers")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn exchanges_mic_eod() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("exchanges/XNAS/eod")
            .add_query_params(&[("limit", "5")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Exchanges::builder()
            .mic("XNAS")
            .eod(Eod::builder().limit(5).unwrap().build().unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn exchanges_mic_eod_latest() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("exchanges/XNAS/eod/latest")
            .add_query_params(&[("limit", "5")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Exchanges::builder()
            .mic("XNAS")
            .eod(Eod::builder().latest(true).build().unwrap())
            .limit(5)
            .unwrap()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn exchanges_mic_eod_date() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("exchanges/XNAS/eod/2023-05-05")
            .add_query_params(&[("limit", "5")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Exchanges::builder()
            .mic("XNAS")
            .eod(
                Eod::builder()
                    .date(NaiveDate::from_ymd_opt(2023, 5, 5).unwrap())
                    .build()
                    .unwrap(),
            )
            .limit(5)
            .unwrap()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
