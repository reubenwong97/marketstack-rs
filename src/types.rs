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

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::{EodData, SplitsData};

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
}
