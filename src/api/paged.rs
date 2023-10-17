use thiserror::Error;

use crate::api::ApiError;

/// New-type implementation reflecting pagination limits.
#[derive(Clone, Debug)]
pub struct PageLimit(pub u16);

impl PageLimit {
    /// Construct PageLimit type with appropriate checks on valid bounds.
    pub fn new(limit: u16) -> Result<Self, ApiError<PaginationError>> {
        if limit <= 1000 {
            Ok(Self(limit))
        } else {
            Err(ApiError::Pagination {
                source: PaginationError::ExceedLimit,
            })
        }
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PaginationError {
    #[error("pagination exceeds limit error")]
    ExceedLimit,
}
