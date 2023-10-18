// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.
use thiserror::Error;

use crate::api::{self, Query};
use crate::api::{eod, AsyncQuery};
use crate::types::EodData;

/// A Marketstack API token.
///
/// Marketstack only supports one kind of token.
#[derive(Debug, Clone)]
pub enum Auth {
    /// A personal access token, obtained through Marketstack dashboard.
    Token(String),
}

impl Auth {
    pub fn check_connection<C>(&self, api: &C) -> Result<(), api::ApiError<C::Error>>
    where
        C: api::Client,
    {
        let _: EodData = eod::Eod::builder()
            .symbol("AAPL")
            .build()
            .unwrap()
            .query(api)?;

        Ok(())
    }

    pub async fn check_connection_async<C>(&self, api: &C) -> Result<(), api::ApiError<C::Error>>
    where
        C: api::AsyncClient + Sync,
    {
        let _: EodData = eod::Eod::builder()
            .symbol("AAPL")
            .build()
            .unwrap()
            .query_async(api)
            .await?;

        Ok(())
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthError {
    #[error("missing auth error")]
    MissingAuth,
}
