// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeSet;

use chrono::NaiveDate;
use derive_builder::Builder;

use crate::api::common::SortOrder;
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// Query for eod.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Eod<'a> {
    /// Search for eod for a symbol.
    #[builder(setter(into), default)]
    symbols: BTreeSet<Cow<'a, str>>,
    /// Exchange to filer symbol by.
    #[builder(setter(into), default)]
    exchange: Option<Cow<'a, str>>,
    /// The sort order for the return results.
    #[builder(default)]
    sort: Option<SortOrder>,
    /// Date to query EOD data from.
    #[builder(default)]
    date_from: Option<NaiveDate>,
    /// Date to query EOD date to.
    #[builder(default)]
    date_to: Option<NaiveDate>,
}

impl<'a> Eod<'a> {
    pub fn builder() -> EodBuilder<'a> {
        EodBuilder::default()
    }
}

impl<'a> EodBuilder<'a> {
    pub fn search_symbols<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = Cow<'a, str>>,
    {
        self.symbols.get_or_insert_with(BTreeSet::new).extend(iter);
        self
    }
}

impl<'a> Endpoint for Eod<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "eod".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
    }
}
