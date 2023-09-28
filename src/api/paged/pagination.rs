// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

use thiserror::Error;

use crate::api::paged::LinkHeaderParseError;

/// Errors which may occur with pagination.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PaginationError {
    /// A `Link` HTTP header can fail to parse.
    #[error("failed to parse a Link HTTP header: {}", source)]
    LinkHeader {
        /// The source of the error.
        #[from]
        source: LinkHeaderParseError,
    },
    /// An invalid URL can be returned.
    #[error("failed to parse a Link HTTP header URL: {}", source)]
    InvalidUrl {
        /// The source of the error.
        #[from]
        source: url::ParseError,
    },
}

// TODO: Confirm pagination options for Marketstack
/// Pagination options for Marketstack.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pagination {
    /// Return all results.
    All,
    /// Limit to a number of results.
    Limit(usize),
}

#[allow(clippy::derivable_impls)]
impl Default for Pagination {
    fn default() -> Self {
        // XXX(rust-1.62): use `#[default]`
        Pagination::All
    }
}

// TODO: determine max page size
const MAX_PAGE_SIZE: usize = 1000;

impl Pagination {
    pub(crate) fn page_limit(self) -> usize {
        match self {
            Pagination::All => MAX_PAGE_SIZE,
            Pagination::Limit(size) => size.min(MAX_PAGE_SIZE),
        }
    }

    pub(crate) fn is_last_page(self, last_page_size: usize, num_results: usize) -> bool {
        // If the last page has fewer elements than our limit, we're definitely done.
        if last_page_size < self.page_limit() {
            return true;
        }

        // Otherwise, check if we have results which fill our limit.
        if let Pagination::Limit(limit) = self {
            return limit <= num_results;
        }

        // We're not done yet.
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::api::Pagination;

    #[test]
    fn pagination_default() {
        assert_eq!(Pagination::default(), Pagination::All);
    }
}
