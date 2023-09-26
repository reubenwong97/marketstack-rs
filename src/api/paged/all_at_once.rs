// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use http::{header, Request};
use serde::de::DeserializeOwned;

use crate::api::paged::link_header;
use crate::api::{
    query, ApiError, AsyncClient, AsyncQuery, Client, Endpoint, Pageable, Pagination, Query,
};

/// A qeury modifier that paginates an endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Paged<E> {
    pub(in crate::api::paged) endpoint: E,
    pub(in crate::api::paged) pagination: Pagination,
}

/// Collect data from a paged endpoint.
pub fn paged<E>(endpoint: E, pagination: Pagination) -> Paged<E> {
    Paged {
        endpoint,
        pagination,
    }
}

impl<E, T, C> Query<Vec<T>, C> for Paged<E>
where
    E: Endpoint,
    E: Pageable,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<Vec<T>, ApiError<<C>::Error>> {
        self.iter(client).collect()
    }
}

#[async_trait]
impl<E, T, C> AsyncQuery<Vec<T>, C> for Paged<E>
where
    E: Endpoint + Sync,
    E: Pageable,
    T: DeserializeOwned + Send + 'static,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<Vec<T>, ApiError<C::Error>> {
        let url = {
            let mut url = client.rest_endpoint(&self.endpoint.endpoint())?;
            self.endpoint.parameters().add_to_url(&mut url);
            url
        };

        let mut page_num = 1;
        let per_page = self.pagination.page_limit();
        let per_page_str = per_page.to_string();

        let results = Arc::new(Mutex::new(Vec::new()));
        let mut next_url = None;
        let use_keyset_pagination = self.endpoint.use_keyset_pagination();

        let body = self.endpoint.body()?;

        loop {
            let page_url = if let Some(url) = next_url.take() {
                url
            } else {
                let page_str = page_num.to_string();
                let mut page_url = url.clone();

                {
                    let mut pairs = page_url.query_pairs_mut();
                    pairs.append_pair("per_page", &per_page_str);

                    if use_keyset_pagination {
                        pairs.append_pair("pagination", "keyset");
                    } else {
                        pairs.append_pair("page", &page_str);
                    }
                }

                page_url
            };

            let req = Request::builder()
                .method(self.endpoint.method())
                .uri(query::url_to_http_uri(page_url));
            let (req, data) = if let Some((mime, data)) = body.as_ref() {
                let req = req.header(header::CONTENT_TYPE, *mime);
                (req, data.clone())
            } else {
                (req, Vec::new())
            };
            let rsp = client.rest_async(req, data).await?;
            let status = rsp.status();

            if use_keyset_pagination {
                next_url = link_header::next_page_from_headers(rsp.headers())?;
            }

            let v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
                v
            } else {
                return Err(ApiError::server_error(status, rsp.body()));
            };
            if !status.is_success() {
                return Err(ApiError::from_marketstack(v));
            }

            let page =
                serde_json::from_value::<Vec<T>>(v).map_err(ApiError::data_type::<Vec<T>>)?;
            let page_len = page.len();

            // Gitlab used to have issues returning paginated results; these have been fixed since,
            // but if it is needed, the bug manifests as Gitlab returning *all* results instead of
            // just the requested results. This can cause an infinite loop here if the number of
            // total results is exactly equal to `per_page`.
            let is_last_page = {
                let mut locked_results = results.lock().expect("poisoned results");
                locked_results.extend(page);
                self.pagination.is_last_page(page_len, locked_results.len())
            };
            if is_last_page {
                break;
            }

            if use_keyset_pagination {
                if next_url.is_none() {
                    break;
                }
            } else {
                page_num += 1;
            }
        }

        let mut locked_results = results.lock().expect("poisoned results");
        Ok(std::mem::take(&mut locked_results))
    }
}
