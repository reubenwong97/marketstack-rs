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

#[cfg(test)]
mod tests {
    use super::PageLimit;

    #[test]
    fn test_new() {
        let limit = PageLimit::new(5);
        assert!(limit.is_ok());

        assert_eq!(limit.unwrap().0, 5);
    }

    #[test]
    fn test_over_limit() {
        let limit = PageLimit::new(9999);
        assert!(limit.is_err());

        let err_message = limit.err().unwrap();
        assert_eq!(
            err_message.to_string(),
            "pagination error: pagination exceeds limit error"
        );
    }
}
