//! Contains Rust types of deserialized responses from Marketstack REST API.

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Pagination Information returned by Marketstack API.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaginationInfo {
    /// Page limit.
    pub limit: u64,
    /// Page offset.
    pub offset: u64,
    /// Results count on the current page.
    pub count: u64,
    /// Total count of the results available.
    pub total: u64,
}

/// Rust representation of single data item from Marketstack `eod` response.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EodDataItem {
    /// Exact date/time the given data was collected in ISO-8601 format.
    pub date: DateTime<Utc>,
    /// Stock ticker symbol of the current data object.
    pub symbol: String,
    /// Exchange MIC identification associated with the current data object.
    pub exchange: String,
    /// Split factor used to adjust prices when a company splits, reverse splits or pays a
    /// distribution.
    pub split_factor: f64,
    /// Distribution of earnings to shareholders.
    pub dividend: f64,
    /// Raw opening price of the given stock ticker.
    pub open: f64,
    /// Raw high price of the given stock ticker.
    pub high: f64,
    /// Raw low price of the given stock ticker.
    pub low: f64,
    /// Raw closing price of the given stock ticker.
    pub close: f64,
    /// Raw volume of the given stock ticker.
    pub volume: f64,
    /// Adjusted opening price of the given stock ticker.
    pub adj_open: f64,
    /// Adjusted high price of the given stock ticker.
    pub adj_high: f64,
    /// Adjusted low price of the given stock ticker.
    pub adj_low: f64,
    /// Adjusted closing price of the given stock ticker.
    pub adj_close: f64,
    /// Adjusted volume of the given stock ticker.
    pub adj_volume: f64,
}

/// Rust representation of the JSON response from `eod` marketstack endpoint.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EodData {
    /// Corresponds to pagination entry from JSON response from marketstack.
    pub pagination: PaginationInfo,
    /// Corresponds to data entry from JSON response from marketstack.
    pub data: Vec<EodDataItem>,
}

/// Rust representation of single data item from Marketstack `splits` response.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SplitsDataItem {
    /// Exact date/time the given data was collected in ISO-8601 format.
    pub date: NaiveDate,
    /// Split factor for that symbol on the date.
    pub split_factor: f64,
    /// Stock ticker symbol of the current data object.
    pub symbol: String,
}

/// Rust representation of the JSON response from `splits` marketstack endpoint.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SplitsData {
    /// Corresponds to pagination entry from JSON response from marketstack.
    pub pagination: PaginationInfo,
    /// Corresponds to data entry from JSON response from marketstack.
    pub data: Vec<SplitsDataItem>,
}

/// Rust representation of single data item from Marketstack `dividends` response.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DividendsDataItem {
    /// Exact date/time the given data was collected in ISO-8601 format.
    pub date: NaiveDate,
    /// Dividend for that symbol on the date.
    pub dividend: f64,
    /// Stock ticker symbol of the current data object.
    pub symbol: String,
}

/// Rust representation of the JSON response from `dividends` marketstack endpoint.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DividendsData {
    /// Corresponds to pagination entry from JSON response from marketstack.
    pub pagination: PaginationInfo,
    /// Corresponds to data entry from JSON response from marketstack.
    pub data: Vec<DividendsDataItem>,
}

/// Rust representation of single data item from Marketstack `currencies` response.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrenciesDataItem {
    /// 3-letter code of the given currency.
    pub code: String,
    /// Name of the given currency.
    pub name: String,
    /// Text symbol of the given currency.
    pub symbol: String,
}

/// Rust representation of the JSON response from `currencies` marketstack endpoint.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrenciesData {
    /// Corresponds to pagination entry from JSON response from marketstack.
    pub pagination: PaginationInfo,
    /// Corresponds to data entry from JSON response from marketstack.
    pub data: Vec<CurrenciesDataItem>,
}

/// Rust representation of single data item from Marketstack `timezones` response.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimezonesDataItem {
    /// Name of the given timezone.
    pub timezone: String,
    /// Abbreviation of the given timezone.
    pub abbr: String,
    /// Summer time abbreviation of the given timezone.
    pub abbr_dst: String,
}

/// Rust representation of the JSON response from `timezones` marketstack endpoint.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimezonesData {
    /// Corresponds to pagination entry from JSON response from marketstack.
    pub pagination: PaginationInfo,
    /// Corresponds to data entry from JSON response from marketstack.
    pub data: Vec<TimezonesDataItem>,
}

/// Rust represenation of a stock exchange.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StockExchange {
    /// Name of the stock exchange associated with the given stock ticker.
    pub name: String,
    /// Name of the stock exchange associated with the given stock ticker.
    pub acronym: String,
    /// MIC identification of the stock exchange associated with the given stock ticker.
    pub mic: String,
    /// Country of the stock exchange associated with the given stock ticker.
    pub country: String,
    /// 3-letter country code of the stock exchange associated with the given stock ticker.
    pub country_code: String,
    /// City of the stock exchange associated with the given stock ticker.
    pub city: String,
    /// Website URL of the stock exchange associated with the given stock ticker.
    pub website: String,
}

/// Rust representation of a single data item from Marketstack `tickers` response.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TickersDataItem {
    /// Name of the given stock ticker.
    pub name: String,
    /// Symbol of the given stock ticker.
    pub symbol: String,
    /// Whether intraday data is available for the stock ticker.
    pub has_intraday: bool,
    /// Whether eod data is available for the stock ticker.
    pub has_eod: bool,
    /// Country ticker is traded in - if available, else `None`.
    pub country: Option<String>,
    /// Stock exchange the ticker is traded in.
    pub stock_exchange: StockExchange,
}

/// Rust representation of the JSON response from `tickers` marketstack endpoint.
pub struct TickersData {
    /// Corresponds to pagination entry from JSON response from marketstack.
    pub pagination: PaginationInfo,
    /// Corresponds to data entry from JSON response from marketstack.
    pub data: Vec<TickersDataItem>,
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::{CurrenciesData, DividendsData, EodData, SplitsData, TimezonesData};

    #[test]
    fn test_deserialize_eod() {
        let json_data = r#"{
        "pagination": {
            "limit": 100,
            "offset": 0,
            "count": 100,
            "total": 9944
        },
        "data": [
            {
                "open": 129.8,
                "high": 133.04,
                "low": 129.47,
                "close": 132.995,
                "volume": 106686703.0,
                "adj_high": 133.04,
                "adj_low": 129.47,
                "adj_close": 132.995,
                "adj_open": 129.8,
                "adj_volume": 106686703.0,
                "split_factor": 1.0,
                "dividend": 0.0,
                "symbol": "AAPL",
                "exchange": "XNAS",
                "date": "2021-04-09T00:00:00+0000"
            }
        ]
    }"#;

        let eod_data: EodData = serde_json::from_str(json_data).unwrap();
        assert_eq!(eod_data.data[0].open, 129.8);
        assert_eq!(eod_data.data[0].symbol, "AAPL");
        assert_eq!(eod_data.pagination.limit, 100);
    }

    #[test]
    fn test_deserialize_splits() {
        let json_data = r#"{
            "pagination": {
              "limit": 100,
              "offset": 0,
              "count": 5,
              "total": 5
            },
            "data": [
              {
                "date": "2020-08-31",
                "split_factor": 4,
                "symbol": "AAPL"
              },
              {
                "date": "2014-06-09",
                "split_factor": 7,
                "symbol": "AAPL"
              },
              {
                "date": "2005-02-28",
                "split_factor": 2,
                "symbol": "AAPL"
              },
              {
                "date": "2000-06-21",
                "split_factor": 2,
                "symbol": "AAPL"
              },
              {
                "date": "1987-06-16",
                "split_factor": 2,
                "symbol": "AAPL"
              }
            ]
          }"#;

        let splits_data: SplitsData = serde_json::from_str(json_data).unwrap();
        assert_eq!(splits_data.data[0].split_factor, 4.0);
        assert_eq!(
            splits_data.data[0].date,
            NaiveDate::from_ymd_opt(2020, 8, 31).unwrap()
        );
        assert_eq!(splits_data.data[0].symbol, "AAPL");

        assert_eq!(splits_data.data[4].split_factor, 2.0);
        assert_eq!(
            splits_data.data[4].date,
            NaiveDate::from_ymd_opt(1987, 6, 16).unwrap()
        );
        assert_eq!(splits_data.data[4].symbol, "AAPL");
    }

    #[test]
    fn test_deserialize_dividends() {
        let json_data = r#"{
            "pagination": {
              "limit": 5,
              "offset": 0,
              "count": 5,
              "total": 68
            },
            "data": [
              {
                "date": "2023-08-11",
                "dividend": 0.24,
                "symbol": "AAPL"
              },
              {
                "date": "2023-05-12",
                "dividend": 0.24,
                "symbol": "AAPL"
              },
              {
                "date": "2023-02-10",
                "dividend": 0.23,
                "symbol": "AAPL"
              },
              {
                "date": "2022-12-23",
                "dividend": 0.17,
                "symbol": "AAPL"
              },
              {
                "date": "2022-11-04",
                "dividend": 0.23,
                "symbol": "AAPL"
              }
            ]
          }"#;

        let dividends_data: DividendsData = serde_json::from_str(json_data).unwrap();
        assert_eq!(dividends_data.pagination.limit, 5);
        assert_eq!(dividends_data.data[0].dividend, 0.24);
        assert_eq!(
            dividends_data.data[0].date,
            NaiveDate::from_ymd_opt(2023, 8, 11).unwrap()
        );
    }

    #[test]
    fn test_deserialize_currencies() {
        let json_data = r#"{
            "pagination": {
              "limit": 3,
              "offset": 0,
              "count": 3,
              "total": 42
            },
            "data": [
              {
                "code": "USD",
                "symbol": "$",
                "name": "US Dollar"
              },
              {
                "code": "ARS",
                "symbol": "AR$",
                "name": "Argentine Peso"
              },
              {
                "code": "EUR",
                "symbol": "€",
                "name": "Euro"
              }
            ]
          }"#;

        let currencies_data: CurrenciesData = serde_json::from_str(json_data).unwrap();
        assert_eq!(currencies_data.pagination.limit, 3);
        assert_eq!(currencies_data.data[0].code, "USD");
        assert_eq!(currencies_data.data[0].symbol, "$");
        assert_eq!(currencies_data.data[0].name, "US Dollar");
    }

    #[test]
    fn test_deserialize_timezones() {
        let json_data = r#"{
            "pagination": {
              "limit": 3,
              "offset": 0,
              "count": 3,
              "total": 57
            },
            "data": [
              {
                "timezone": "America/New_York",
                "abbr": "EST",
                "abbr_dst": "EDT"
              },
              {
                "timezone": "America/Argentina/Buenos_Aires",
                "abbr": "-03",
                "abbr_dst": "-03"
              },
              {
                "timezone": "Europe/Vienna",
                "abbr": "CET",
                "abbr_dst": "CEST"
              }
            ]
          }"#;

        let timezones_data: TimezonesData = serde_json::from_str(json_data).unwrap();
        assert_eq!(timezones_data.data[0].timezone, "America/New_York");
        assert_eq!(timezones_data.data[0].abbr, "EST");
        assert_eq!(timezones_data.data[0].abbr_dst, "EDT");
    }
}
