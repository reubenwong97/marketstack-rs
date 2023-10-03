// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

use crate::api::endpoint_prelude::*;
use crate::auth::Auth;

/// A `auth` modifier that can be applied to any endpoint.
#[derive(Clone)]
pub struct AuthContext {
    /// The auth token to use for the endpoint.
    token: Auth,
}
