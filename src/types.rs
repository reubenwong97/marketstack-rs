use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

/// Basic struct that acts as dummy.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicPublic {}
