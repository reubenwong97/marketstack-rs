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
