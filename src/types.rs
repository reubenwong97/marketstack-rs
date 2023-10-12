use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Basic struct that acts as dummy.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicPublic {}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EodDataItem {
    /// Exact date/time the given data was collected in ISO-8601 format.
    date: NaiveDate,
    /// Stock ticker symbol of the current data object.
    symbol: String,
    /// Exchange MIC identification associated with the current data object.
    exchange: String,
    /// Split factor used to adjust prices when a company splits, reverse splits or pays a
    /// distribution.
    split_factor: f64,
    /// Distribution of earnings to shareholders.
    dividend: f64,
    /// Raw opening price of the given stock ticker.
    open: f64,
    /// Raw high price of the given stock ticker.
    high: f64,
    /// Raw low price of the given stock ticker.
    low: f64,
    /// Raw closing price of the given stock ticker.
    close: f64,
    /// Raw volume of the given stock ticker.
    volume: f64,
    /// Adjusted opening price of the given stock ticker.
    adj_open: f64,
    /// Adjusted high price of the given stock ticker.
    adj_high: f64,
    /// Adjusted low price of the given stock ticker.
    adj_low: f64,
    /// Adjusted closing price of the given stock ticker.
    adj_close: f64,
    /// Adjusted volume of the given stock ticker.
    adj_volume: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EodData {
    pub pagination_info: PaginationInfo,
    pub data: Vec<EodDataItem>,
}
