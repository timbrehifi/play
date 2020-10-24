// Copyright 2020 Nicholas Young. All rights reserved.
//
// Use of this source code is governed by the Mozilla Public License,
// version 2.0, which can be found in the included `LICENSE` file,
// or at https://www.mozilla.org/en-US/MPL/2.0.

use std::{convert::From, io};

/// Type alias representing fallible operations.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Errors that may arise from application operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Environment {0}")]
    Env(envy::Error),
    #[error("Unresovable path")]
    UnresolvablePath,
    #[error("Encodings other than UTF-8 are unsupported")]
    BadEncoding,
    #[error("TOML {0}")]
    Toml(toml::de::Error),
    #[error("I/O {0}")]
    Io(io::Error),
}

impl From<envy::Error> for Error {
    fn from(e: envy::Error) -> Self {
        Self::Env(e)
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Self::Toml(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}
