// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.
use thiserror::Error;

use crate::api::BasicEndpoint;
use crate::api::{self, endpoint_prelude::*, Query};
use crate::types::BasicPublic;

/// A Marketstack API token.
///
/// Marketstack only supports one kind of token.
#[derive(Clone)]
pub enum Auth {
    /// A personal access token, obtained through Marketstack dashboard.
    Token(String),
}

impl Auth {
    pub fn check_connection<C>(&self, api: &C) -> Result<(), api::ApiError<C::Error>>
    where
        C: api::Client,
    {
        let _: BasicPublic = BasicEndpoint::builder().build().unwrap().query(api)?;

        Ok(())
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthError {
    #[error("missing auth error")]
    MissingAuth,
}
