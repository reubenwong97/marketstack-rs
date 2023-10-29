//! Implemented for `tickers` and associated endpoints.

use std::borrow::Cow;

use derive_builder::Builder;

use crate::api::dividends::Dividends;
use crate::api::eod::Eod;
use crate::api::paged::PaginationError;
use crate::api::splits::Splits;
use crate::api::{endpoint_prelude::*, ApiError};

/// Base for `tickers`.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option), build_fn(validate = "Self::validate"))]
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
    /// `Splits` struct being built, and held by the `Tickers` struct.
    #[builder(setter(into), default)]
    splits: Option<Splits<'a>>,
    /// `Dividends` struct being built, and held by the `Tickers` struct.
    #[builder(setter(into), default)]
    dividends: Option<Dividends<'a>>,
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

            // NOTE: validator will ensure only one can be active.
            if let Some(eod) = &self.eod {
                endpoint.push_str(&format!("/{}", eod.endpoint().as_ref()));
            }
            if let Some(splits) = &self.splits {
                endpoint.push_str(&format!("/{}", splits.endpoint().as_ref()));
            }
            if let Some(dividends) = &self.dividends {
                endpoint.push_str(&format!("/{}", dividends.endpoint().as_ref()));
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
        if let Some(splits) = &self.splits {
            params = splits.parameters().clone();
        }
        if let Some(dividends) = &self.dividends {
            params = dividends.parameters().clone();
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

    //// Check that `Tickers` contains valid endpoint combinations.
    fn validate(&self) -> Result<(), String> {
        let active_fields = [
            self.eod.is_some(),
            self.splits.is_some(),
            self.dividends.is_some(),
        ];
        let count = active_fields.iter().filter(|x| **x).count();
        if count > 1 {
            Err("Invalid combinations of `eod`, `splits` or `dividends`".into())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {

    use chrono::NaiveDate;

    use crate::api::common::SortOrder;
    use crate::api::dividends::Dividends;
    use crate::api::eod::Eod;
    use crate::api::splits::Splits;
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

    #[test]
    fn tickers_splits() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("tickers/AAPL/splits")
            .add_query_params(&[("date_from", "2023-09-27"), ("date_to", "2023-09-30")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Tickers::builder()
            .ticker("AAPL")
            .splits(
                Splits::builder()
                    .date_from(NaiveDate::from_ymd_opt(2023, 9, 27).unwrap())
                    .date_to(NaiveDate::from_ymd_opt(2023, 9, 30).unwrap())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn tickers_dividends() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("tickers/AAPL/dividends")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Tickers::builder()
            .ticker("AAPL")
            .dividends(Dividends::builder().build().unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn tickers_validator() {
        let endpoint = Tickers::builder()
            .eod(Eod::builder().build().unwrap())
            .splits(Splits::builder().build().unwrap())
            .build();
        assert!(endpoint.is_err());
        assert!(endpoint.err().unwrap().to_string().contains("Invalid"));
    }
}
