[![Crates.io](https://img.shields.io/crates/v/marketstack
)](https://crates.io/crates/marketstack)
[![Documentation](https://img.shields.io/docsrs/marketstack/latest
)](https://docs.rs/marketstack)
[![Codecov](https://codecov.io/gh/reubenwong97/marketstack-rs/graph/badge.svg?token=2RHYDZWTCL)](https://codecov.io/gh/reubenwong97/marketstack-rs)
[![Builds](https://img.shields.io/github/actions/workflow/status/reubenwong97/marketstack-rs/general.yml
)](https://github.com/reubenwong97/marketstack-rs)

For more details about this crate and examples, please check out the [documentation](https://docs.rs/marketstack/latest/marketstack/).

# Marketstack API

```rust
use marketstack::eod::Eod;
use marketstack::Marketstack;
use marketstack::EodData;
use marketstack::{self, Query};

// Create the http client. If you are a paid user, use `Marketstack::new()`.
let client = Marketstack::new_insecure("api.marketstack.com", "private-token").unwrap();

// Create endpoint to query end of day data for AAPL. Corresponds to `eod` endpoint.
let endpoint = Eod::builder().symbol("AAPL").build().unwrap();

// Call the endpoint and retrieve data from Marketstack. Data is deserialized
// for you into a Rust type: `EodData`.
let eod_data: EodData = endpoint.query(&client).unwrap();

// Some endpoints support pagination. Since Marketstack does pagination through query
// params, we simply specify them in the endpoint builder.
// Note that there are limits defined, and therefore, limit(5) is fallible and returns
// a Result.
let pageable_endpoint = Eod::builder().symbol("AAPL").limit(5).unwrap().build().unwrap();
```

## About Marketstack-rs

This library implements an interface to communicate with the [Marketstack](https://marketstack.com/) API. Not all endpoints are implemented (some are paid users only of which I am not), but pull requests are welcome.

This API binding is based off Marketstack's v1 API.

The endpoints that are supported all live under the [`api`](https://github.com/reubenwong97/marketstack-rs/tree/master/src/api) module. Each endpoint may be constructed conveniently using a "builder" pattern (in this case, the [`derive_builder`](https://crates.io/crates/derive_builder) crate makes implementing this very ergonomic) to provide supported fields. To use an endpoint, you may query it using the [`Query`](https://github.com/reubenwong97/marketstack-rs/blob/master/src/api/query.rs) trait.

All endpoints return data types of the caller's choosing that implement `serde`'s `Deserialize` trait. Callers should define their own structures for obtaining data from the API, though most have already been implemented for the caller. However, this provides flexibility to allow the user to adapt to newer releases of Marketstack, or to easily implement data queried from endpoints that have yet to be implemented by this crate. The user does not need to wait for this crate to implement any endpoints.

## FAQs

### Is this production ready?

This crate evolved out of a need from a personal project to pull data from an affordable / free data provider. I noticed that one had not been created and therefore implemented this myself. While I have begun using it, I have not yet extensively tested it and certain paid endpoints could not be implemented as I did not have access and could not discern their structure from the documentation.

Therefore, I would caution users against using this for production-level projects. That said, feel free to raise issues, create pull requests or fork this repo.
