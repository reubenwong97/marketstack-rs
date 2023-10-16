//! A library for communicating with the Marketstack REST API.

pub mod api;
mod auth;
mod marketstack;
pub mod types;

pub use crate::auth::AuthError;
pub use crate::marketstack::{
    AsyncMarketstack, Marketstack, MarketstackBuilder, MarketstackError, RestError,
};
pub use crate::types::*;

#[cfg(test)]
mod test;
