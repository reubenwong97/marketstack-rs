//! Implemented for `tickers` and associated endpoints.

use std::borrow::Cow;

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;
use crate::api::eod::Eod;

/// Base for `tickers`.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Tickers<'a> {
    /// Ticker symbol.
    #[builder(setter(into), default)]
    ticker: Option<Cow<'a, str>>,
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
}

#[cfg(test)]
mod tests {

    use chrono::NaiveDate;

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
}
