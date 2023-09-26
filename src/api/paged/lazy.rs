// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::sync::RwLock;

use async_trait::async_trait;
use bytes::Bytes;
use futures_util::Stream;
use http::request::Builder as RequestBuilder;
use http::{header, Request, Response};
use query::AsyncQuery;
use serde::de::DeserializeOwned;
use url::Url;

use crate::api::paged::link_header;
use crate::api::{
    query, ApiError, AsyncClient, Client, Endpoint, Pageable, Paged, Query, RestClient,
};

impl<E> Paged<E>
where
    E: Endpoint,
    E: Pageable,
{
    /// Create an iterator over the results of paginated results for with a client.
    pub fn iter<'a, C, T>(&'a self, client: &'a C) -> LazilyPagedIter<'a, E, C, T> {
        LazilyPagedIter::new(self, client)
    }
}

impl<E> Paged<E>
where
    E: Endpoint + Pageable + Sync,
{
    /// Create a stream over the results of paginated results for with a client.
    pub fn iter_async<'a, C, T>(
        &'a self,
        client: &'a C,
    ) -> impl Stream<Item = Result<T, ApiError<C::Error>>> + 'a
    where
        T: DeserializeOwned + 'static,
        C: AsyncClient + Sync,
    {
        let iter = LazilyPagedIter::new(self, client);
        futures_util::stream::unfold(iter, |mut iter| async move {
            iter.next_async().await.map(|item| (item, iter))
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum KeysetPage {
    First,
    Next(Url),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Page {
    Number(u64),
    Keyset(KeysetPage),
    Done,
}

impl Page {
    fn next_url(&self) -> Option<&Url> {
        if let Self::Keyset(KeysetPage::Next(url)) = self {
            Some(url)
        } else {
            None
        }
    }

    fn next_page(&mut self, next_url: Option<Url>) {
        let next_page = match *self {
            Self::Number(page) => Self::Number(page + 1),
            Self::Keyset(_) => {
                if let Some(next_url) = next_url {
                    Self::Keyset(KeysetPage::Next(next_url))
                } else {
                    Self::Done
                }
            }
            Self::Done => Self::Done,
        };

        *self = next_page;
    }

    fn apply_to(&self, pairs: &mut url::form_urlencoded::Serializer<url::UrlQuery>) {
        match self {
            Self::Number(page) => {
                let page_str = page.to_string();
                pairs.append_pair("page", &page_str);
            }
            Self::Keyset(_) => {
                pairs.append_pair("pagination", "keyset");
            }
            Self::Done => {
                unreachable!("The `Done` state should not be applied to any url")
            }
        }
    }
}

struct PageState {
    total_results: usize,
    next_page: Page,
}

struct LazilyPagedState<'a, E> {
    paged: &'a Paged<E>,
    page_state: RwLock<PageState>,
}

impl<'a, E> LazilyPagedState<'a, E>
where
    E: Pageable,
{
    fn new(paged: &'a Paged<E>) -> Self {
        let next_page = if paged.endpoint.use_keyset_pagination() {
            Page::Keyset(KeysetPage::First)
        } else {
            Page::Number(1)
        };

        let page_state = PageState {
            total_results: 0,
            next_page,
        };

        Self {
            paged,
            page_state: RwLock::new(page_state),
        }
    }
}

impl<'a, E> LazilyPagedState<'a, E> {
    fn next_page(&self, last_page_size: usize, next_url: Option<Url>) {
        let mut page_state = self.page_state.write().expect("poisoned next_page");
        page_state.total_results += last_page_size;

        if self
            .paged
            .pagination
            .is_last_page(last_page_size, page_state.total_results)
        {
            page_state.next_page = Page::Done;
        } else {
            page_state.next_page.next_page(next_url);
        }
    }
}

impl<'a, E> LazilyPagedState<'a, E>
where
    E: Endpoint,
{
    fn page_url<C>(&self, client: &C) -> Result<Option<Url>, ApiError<C::Error>>
    where
        C: RestClient,
    {
        let page_state = self.page_state.read().expect("poisoned next_page");
        let next_page = &page_state.next_page;

        if *next_page == Page::Done {
            return Ok(None);
        }

        let url = if let Some(next_url) = next_page.next_url() {
            next_url.clone()
        } else {
            let mut url = client.rest_endpoint(&self.paged.endpoint.endpoint())?;
            self.paged.endpoint.parameters().add_to_url(&mut url);

            let per_page = self.paged.pagination.page_limit();
            let per_page_str = per_page.to_string();

            {
                let mut pairs = url.query_pairs_mut();
                pairs.append_pair("per_page", &per_page_str);

                next_page.apply_to(&mut pairs);
            }

            url
        };

        Ok(Some(url))
    }

    fn build_request<C>(&self, url: Url) -> Result<(RequestBuilder, Vec<u8>), ApiError<C::Error>>
    where
        C: RestClient,
    {
        let body = self.paged.endpoint.body()?;

        let req = Request::builder()
            .method(self.paged.endpoint.method())
            .uri(query::url_to_http_uri(url));
        Ok(if let Some((mime, data)) = body.as_ref() {
            let req = req.header(header::CONTENT_TYPE, *mime);
            (req, data.clone())
        } else {
            (req, Vec::new())
        })
    }

    fn process_response<C, T>(&self, rsp: Response<Bytes>) -> Result<Vec<T>, ApiError<C::Error>>
    where
        E: Pageable,
        T: DeserializeOwned,
        C: RestClient,
    {
        let status = rsp.status();

        let next_url = if self.paged.endpoint.use_keyset_pagination() {
            link_header::next_page_from_headers(rsp.headers())?
        } else {
            None
        };

        let v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
            v
        } else {
            return Err(ApiError::server_error(status, rsp.body()));
        };
        if !status.is_success() {
            return Err(ApiError::from_marketstack(v));
        }

        let page = serde_json::from_value::<Vec<T>>(v).map_err(ApiError::data_type::<Vec<T>>)?;
        self.next_page(page.len(), next_url);

        Ok(page)
    }
}

impl<'a, E, T, C> Query<Vec<T>, C> for LazilyPagedState<'a, E>
where
    E: Endpoint,
    E: Pageable,
    T: DeserializeOwned,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<Vec<T>, ApiError<C::Error>> {
        let url = if let Some(url) = self.page_url(client)? {
            url
        } else {
            // Just return empty data.
            // XXX: Return a new kind of PaginationError here?
            return Ok(Vec::new());
        };
        let (req, data) = self.build_request::<C>(url)?;
        let rsp = client.rest(req, data)?;
        self.process_response::<C, _>(rsp)
    }
}

#[async_trait]
impl<'a, E, T, C> AsyncQuery<Vec<T>, C> for LazilyPagedState<'a, E>
where
    E: Endpoint + Pageable + Sync,
    T: DeserializeOwned + 'static,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<Vec<T>, ApiError<C::Error>> {
        let url = if let Some(url) = self.page_url(client)? {
            url
        } else {
            // Just return empty data.
            // XXX: Return a new kind of PaginationError here?
            return Ok(Vec::new());
        };
        let (req, data) = self.build_request::<C>(url)?;
        let rsp = client.rest_async(req, data).await?;
        self.process_response::<C, _>(rsp)
    }
}

/// An iterator which yields items from a paginated result.
///
/// The pages are fetched lazily, so endpoints not using keyset pagination may observe duplicate or
/// missing items (depending on sorting) if new objects are created or removed while iterating.
pub struct LazilyPagedIter<'a, E, C, T> {
    client: &'a C,
    state: LazilyPagedState<'a, E>,
    current_page: Vec<T>,
}

impl<'a, E, C, T> LazilyPagedIter<'a, E, C, T>
where
    E: Endpoint,
    E: Pageable,
{
    fn new(paged: &'a Paged<E>, client: &'a C) -> Self {
        let state = LazilyPagedState::new(paged);

        Self {
            client,
            state,
            current_page: Vec::new(),
        }
    }
}

impl<'a, E, C, T> Iterator for LazilyPagedIter<'a, E, C, T>
where
    E: Endpoint,
    E: Pageable,
    T: DeserializeOwned,
    C: Client,
{
    type Item = Result<T, ApiError<C::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_page.is_empty() {
            self.current_page = match self.state.query(self.client) {
                Ok(data) => data,
                Err(err) => return Some(Err(err)),
            };

            // Reverse the page order so that `.pop()` works.
            self.current_page.reverse();
        }

        self.current_page.pop().map(Ok)
    }
}

// Instead of implementing Stream directly, we implement this "async" next method and use it with
// `stream::unfold` to return an anonymous Stream impl.
impl<'a, E, C, T> LazilyPagedIter<'a, E, C, T>
where
    E: Endpoint + Pageable + Sync,
    T: DeserializeOwned + 'static,
    C: AsyncClient + Sync,
{
    async fn next_async(&mut self) -> Option<Result<T, ApiError<C::Error>>> {
        if self.current_page.is_empty() {
            self.current_page = match self.state.query_async(self.client).await {
                Ok(data) => data,
                Err(err) => return Some(Err(err)),
            };

            // Reverse the page order so that `.pop()` works.
            self.current_page.reverse();
        }

        self.current_page.pop().map(Ok)
    }
}
