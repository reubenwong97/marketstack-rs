// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

use thiserror::Error;

/// Errors which may occur when creating form data.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BodyError {
    /// Body data could not be serialized from form parameters.
    #[error("failed to URL encode form parameters: {}", source)]
    UrlEncoded {
        /// The source of the error.
        #[from]
        source: serde_urlencoded::ser::Error,
    },
}
