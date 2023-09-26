// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

use async_trait::async_trait;
use http::{header, Request};

use crate::api::{query, ApiError, AsyncClient, Client, Endpoint, Query};

/// A query modifier that ignores the data returned from an endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ignore<E> {
    endpoint: E,
}

/// Ignore the resulting data from an endpoint.
pub fn ignore<E>(endpoint: E) -> Ignore<E> {
    Ignore { endpoint }
}

impl<E, C> Query<(), C> for Ignore<E>
where
    E: Endpoint,
    C: Client,
{
    fn query(&self, client: &C) -> Result<(), ApiError<<C>::Error>> {
        let mut url = client.rest_endpoint(&self.endpoint.endpoint())?;
    }
}
