// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;

use http::{self, header, Method, Request};

use crate::api::BodyError;

pub trait Endpoint {
    /// The HTTP method to use for the endpoint.
    fn method(&self) -> Method;
    /// The path to the endpoint.
    fn endpoint(&self) -> Cow<'static, str>;

    /// The body for the endpoint.
    ///
    /// Returns the `Content-Encoding` header for the data as well as the data itself.
    ///
    /// # Errors
    /// This method returns an error if the body could not be serialized to JSON.
    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        Ok(None)
    }
}
