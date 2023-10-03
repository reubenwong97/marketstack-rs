// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

use crate::api::endpoint_prelude::*;

/// A Marketstack API token.
///
/// Marketstack only supports one kind of token.
#[derive(Clone)]
pub enum Auth {
    /// A personal access token, obtained through Marketstack dashboard.
    Token(String),
}

pub fn with_auth<E: Endpoint>(auth: Auth, endpoint: E) -> impl Endpoint {
    endpoint.parameters().push("access_key", auth);

    endpoint
}
