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
use crate::auth::Auth;

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

impl ExpectedUrlBuilder {
    pub fn add_query_params(&mut self, pairs: &[(&'static str, &'static str)]) -> &mut Self {
        self.query
            .get_or_insert_with(Vec::new)
            .extend(pairs.iter().cloned().map(|(k, v)| (k.into(), v.into())));
        self
    }
}

impl ExpectedUrl {
    pub fn builder() -> ExpectedUrlBuilder {
        ExpectedUrlBuilder::default()
    }

    fn check(&self, method: Method, url: &Url) {
        // Test that the method is as expected.
        assert_eq!(method, self.method);

        // Ensure that the URL was not tampered with in the meantime.
        assert_eq!(url.scheme(), "https");
        assert_eq!(url.username(), "");
        assert_eq!(url.password(), None);
        assert_eq!(url.host_str().unwrap(), "marketstack.host.invalid");
        assert_eq!(url.port(), None);
        assert_eq!(url.path(), format!("/v1/{}", self.endpoint));
        let mut count = 0;
        for (ref key, ref value) in url.query_pairs() {
            if self.paginated && Self::is_pagination_key(key) {
                continue;
            }

            let found = self.query.iter().any(|(expected_key, expected_value)| {
                key == expected_key && value == expected_value
            });

            if !found {
                panic!("unexpected query parameter `{}={}`", key, value);
            }
            count += 1;
        }
        assert_eq!(count, self.query.len());
        assert_eq!(url.fragment(), None);
    }

    fn is_pagination_key(key: &str) -> bool {
        key == "pagination" || key == "offset" || key == "limit"
    }
}

#[derive(Debug, Clone)]
struct MockResponse {
    status: StatusCode,
    data: Vec<u8>,
}

impl MockResponse {
    fn response(&self) -> Response<Vec<u8>> {
        Response::builder()
            .status(self.status)
            .body(self.data.clone())
            .unwrap()
    }
}

#[derive(Debug, Default)]
struct MockClient {
    response_map: HashMap<(Method, String), MockResponse>,
}

const CLIENT_STUB: &str = "https://marketstack.host.invalid/v1";

pub struct SingleTestClient {
    client: MockClient,
    expected: ExpectedUrl,
    auth: Auth,
}

impl SingleTestClient {
    pub fn new_raw<T>(expected: ExpectedUrl, data: T) -> Self
    where
        T: Into<Vec<u8>>,
    {
        let mut client = MockClient::default();

        let request = (
            expected.method.clone(),
            format!("/v1/{}", expected.endpoint),
        );
        let response = MockResponse {
            status: expected.status,
            data: data.into(),
        };
        let auth = Auth::Token("123".into());

        client.response_map.insert(request, response);

        Self {
            client,
            expected,
            auth,
        }
    }

    pub fn new_json<T>(expected: ExpectedUrl, data: &T) -> Self
    where
        T: Serialize,
    {
        let data = serde_json::to_vec(data).unwrap();
        Self::new_raw(expected, data)
    }
}

#[derive(Debug, Error)]
#[error("test client error")]
pub enum TestClientError {}

impl RestClient for SingleTestClient {
    type Error = TestClientError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>> {
        Ok(Url::parse(&format!("{}/{}", CLIENT_STUB, endpoint))?)
    }

    fn get_auth(&self) -> Option<Auth> {
        Some(self.auth.clone())
    }
}

impl Client for SingleTestClient {
    fn rest(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>> {
        let url = Url::parse(&format!("{}", request.uri_ref().unwrap())).unwrap();
        self.expected
            .check(request.method_ref().unwrap().clone(), &url);
        assert_eq!(
            &body,
            &self.expected.body,
            "\nbody is not the same:\nactual  : {}\nexpected: {}\n",
            String::from_utf8_lossy(&body),
            String::from_utf8_lossy(&self.expected.body),
        );
        let headers = request.headers_ref().unwrap();
        let content_type = headers
            .get_all(header::CONTENT_TYPE)
            .iter()
            .map(|value| value.to_str().unwrap());

        if let Some(expected_content_type) = self.expected.content_type.as_ref() {
            itertools::assert_equal(content_type, [expected_content_type].iter().cloned());
        } else {
            assert_eq!(content_type.count(), 0);
        }

        let request = request.body(body).unwrap();

        Ok(self
            .client
            .response_map
            .get(&(request.method().clone(), request.uri().path().into()))
            .expect("no matching request found")
            .response()
            .map(Into::into))
    }
}

#[async_trait]
impl AsyncClient for SingleTestClient {
    async fn rest_async(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<<Self as RestClient>::Error>> {
        <Self as Client>::rest(self, request, body)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Page {
    ByNumber { number: usize, size: usize },
}

impl Page {
    fn range(self) -> Range<usize> {
        match self {
            Page::ByNumber { number, size } => {
                // Marketstack offset defaults to 0
                assert_eq!(number, 0);
                let start = size * (number);
                start..start + size
            }
        }
    }
}

pub struct PagedTestClient<T> {
    expected: ExpectedUrl,
    data: Vec<T>,
    auth: Auth,
}

const KEYSET_QUERY_PARAM: &str = "__test_keyset";
const DEFAULT_PAGE_SIZE: usize = 20;

impl<T> RestClient for PagedTestClient<T> {
    type Error = TestClientError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>> {
        Ok(Url::parse(&format!("{}/{}", CLIENT_STUB, endpoint))?)
    }

    fn get_auth(&self) -> Option<Auth> {
        Some(self.auth.clone())
    }
}

impl<T> Client for PagedTestClient<T>
where
    T: Serialize,
{
    fn rest(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>> {
        let url = Url::parse(&format!("{}", request.uri_ref().unwrap())).unwrap();

        self.expected
            .check(request.method_ref().unwrap().clone(), &url);
        assert_eq!(
            &body,
            &self.expected.body,
            "\nbody is not the same:\nactual  : {}\nexpected: {}\n",
            String::from_utf8_lossy(&body),
            String::from_utf8_lossy(&self.expected.body),
        );
        let headers = request.headers_ref().unwrap();
        let content_type = headers
            .get_all(header::CONTENT_TYPE)
            .iter()
            .map(|value| value.to_str().unwrap());
        if let Some(expected_content_type) = self.expected.content_type.as_ref() {
            itertools::assert_equal(content_type, [expected_content_type].iter().cloned());
        } else {
            assert_eq!(content_type.count(), 0);
        }

        let mut pagination = false;
        let mut offset: Option<usize> = None;
        let mut limit = DEFAULT_PAGE_SIZE;

        for (ref key, ref value) in url.query_pairs() {
            match key.as_ref() {
                "pagination" => {
                    assert_eq!(value, "keyset");
                    pagination = true;
                }
                "offset" => {
                    offset = Some(value.parse().unwrap());
                }
                "limit" => {
                    limit = value.parse().unwrap();
                }
                _ => (),
            }
        }

        let offset = Page::ByNumber {
            number: offset.unwrap_or(0),
            size: limit,
        };
        let range = {
            // Limit the range to the amount of data actually available.
            let mut range = offset.range();
            range.end = cmp::min(range.end, self.data.len());
            range
        };

        let request = request.body(body).unwrap();
        assert_eq!(*request.method(), Method::GET);

        let response = Response::builder().status(self.expected.status);
        let response = if pagination {
            if range.end + 1 < self.data.len() {
                // Generate the URL for the next page.
                let next_url = {
                    let mut next_url = url.clone();
                    next_url
                        .query_pairs_mut()
                        .clear()
                        .extend_pairs(
                            url.query_pairs()
                                .filter(|(key, _)| key != KEYSET_QUERY_PARAM),
                        )
                        .append_pair(KEYSET_QUERY_PARAM, &format!("{}", range.end));
                    next_url
                };
                let next_header = format!("<{}>; rel=\"next\"", next_url);
                response.header(http::header::LINK, next_header)
            } else {
                response
            }
        } else {
            response
        };

        let data_page = &self.data[range];

        Ok(response
            .body(serde_json::to_vec(data_page).unwrap())
            .unwrap()
            .map(Into::into))
    }
}

#[async_trait]
impl<T> AsyncClient for PagedTestClient<T>
where
    T: Serialize + Send + Sync,
{
    async fn rest_async(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<<Self as RestClient>::Error>> {
        <Self as Client>::rest(self, request, body)
    }
}
