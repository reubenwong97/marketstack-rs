use derive_builder::Builder;

use crate::api::endpoint_prelude::*;

/// Dummy endpoint that is not tied to a Marketstack endpoint.
#[derive(Debug, Clone, Copy, Builder)]
pub struct BasicEndpoint {}

impl BasicEndpoint {
    /// Create a builder for the endpoint.
    pub fn builder() -> BasicEndpointBuilder {
        BasicEndpointBuilder::default()
    }
}

impl Endpoint for BasicEndpoint {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "".into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::basic::BasicEndpoint;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn defaults_are_sufficient() {
        BasicEndpoint::builder().build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = BasicEndpoint::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
