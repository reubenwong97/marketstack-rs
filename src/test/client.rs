// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::cmp;
use std::collections::HashMap;
use std::ops::Range;

use async_trait::async_trait;
use bytes::Bytes;
use derive_builder::Builder;
use http::request::Builder as RequestBuilder;
use http::{header, Method, Response, StatusCode};
use serde::ser::Serialize;
use thiserror::Error;
use url::Url;

use crate::api::{ApiError, AsyncClient, Client, RestClient};

#[derive(Debug, Builder)]
pub struct ExpectedUrl {
    #[builder(default = "Method::GET")]
    pub method: Method,
    pub endpoint: &'static str,
    #[builder(default)]
    pub query: Vec<(Cow<'static, str>, Cow<'static, str>)>,
    #[builder(setter(strip_option, into), default)]
    pub content_type: Option<String>,
    #[builder(default)]
    pub body: Vec<u8>,
    #[builder(default = "StatusCode::OK")]
    pub status: StatusCode,

    #[builder(default = "false")]
    pub paginated: bool,
}
