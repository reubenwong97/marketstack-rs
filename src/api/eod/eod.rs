// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// Query for eod.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Eod<'a> {
    /// Access key for querying API.
    #[builder(setter(into), default)]
    access_key: Cow<'a, str>,
    /// Search for eod for a symbol.
    #[builder(setter(into), default)]
    symbols: Cow<'a, str>,
    /// Exchange to filer symbol by.
    #[builder(setter(into), default)]
    exchange: Option<Cow<'a, str>>,
}
