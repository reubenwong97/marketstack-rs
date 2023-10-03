// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

/// A Marketstack API token.
///
/// Marketstack only supports one kind of token.
#[derive(Clone)]
pub enum Auth {
    /// A personal access token, obtained through Marketstack dashboard.
    Token(String),
}
