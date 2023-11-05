//! API types common to many endpoints.
//!
//! Usually these are enumerations or other simple wrappers around structures
//! present in Marketstack's REST API.

use std::borrow::Cow;

use crate::api::ParamValue;

/// Orderings for sorted results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    /// Value should be sorted ASC, usually by date.
    Ascending,
    /// Value should be sorted DESC, usually by date.
    Descending,
}

#[allow(clippy::derivable_impls)]
impl Default for SortOrder {
    fn default() -> Self {
        // XXX{rust-1.62): use `#[default]`}
        SortOrder::Descending
    }
}

impl SortOrder {
    /// The string representation of the sort order.
    pub fn as_str(self) -> &'static str {
        match self {
            SortOrder::Ascending => "ASC",
            SortOrder::Descending => "DESC",
        }
    }
}

impl ParamValue<'static> for SortOrder {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Data interval for `intraday` endpoint.
#[derive(Debug, Clone)]
pub enum Interval {
    /// 1min interval.
    OneMinute,
    /// 5min interval.
    FiveMinutes,
    /// 10min interval.
    TenMinutes,
    /// 15min interval.
    FifteenMinutes,
    /// 30min interval.
    ThirtyMinutes,
    /// 1hour interval.
    OneHour,
    /// 3hour interval.
    ThreeHour,
    /// 6hour interval.
    SixHour,
    /// 12hour interval.
    TwelveHour,
    /// 24hour interval.
    TwentyFourHour,
}

#[allow(clippy::derivable_impls)]
impl Default for Interval {
    fn default() -> Self {
        // XXX{rust-1.62): use `#[default]`}
        Interval::OneHour
    }
}

impl Interval {
    /// The string representation of the interval.
    fn as_str(&self) -> &'static str {
        match self {
            Interval::OneMinute => "1min",
            Interval::FiveMinutes => "5min",
            Interval::TenMinutes => "10min",
            Interval::FifteenMinutes => "15min",
            Interval::ThirtyMinutes => "30min",
            Interval::OneHour => "1hour",
            Interval::ThreeHour => "3hour",
            Interval::SixHour => "6hour",
            Interval::TwelveHour => "12hour",
            Interval::TwentyFourHour => "24hour",
        }
    }
}

impl ParamValue<'static> for Interval {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}
