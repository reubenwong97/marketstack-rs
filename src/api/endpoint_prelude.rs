// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! Endpoint prelude
//!
//! This module re-exports all of the types needed for endpoints to implement the
//! [`Endpoint`](../trait.Endpoint.html) trait.

pub use std::borrow::Cow;

pub use http::Method;

pub use crate::api::BodyError;
pub use crate::api::Client;
pub use crate::api::Endpoint;
