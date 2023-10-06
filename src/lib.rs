//! A library for communicating with the Marketstack REST API.

pub mod api;
mod auth;
mod marketstack;
pub mod types;

#[cfg(test)]
mod test;
