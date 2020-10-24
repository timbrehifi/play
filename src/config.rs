// Copyright 2020 Nicholas Young. All rights reserved.
//
// Use of this source code is governed by the Mozilla Public License,
// version 2.0, which can be found in the included `LICENSE` file,
// or at https://www.mozilla.org/en-US/MPL/2.0.

use crate::result::{Error, Result};
use serde::Deserialize;
use std::{env, path::Path, str};
use tokio::{fs, io::AsyncReadExt};

/// Options necessary to start and operate an instance of Play.
#[derive(Debug, Deserialize)]
pub struct Config {
    port: u16,
    database_path: Option<String>,
}

impl Config {
    /// Create a new instance of `Config` with default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create an instance of `Config` from available environment
    /// variables.
    pub fn from_env() -> Result<Self> {
        Ok(envy::from_env::<Self>()?)
    }

    /// Create an instance of `Config` from a configuration file
    /// on disk.
    pub async fn from_file<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = env::current_dir()?.join(path.as_ref());
        let mut file = fs::File::open(&path)
            .await
            .map_err(|_| Error::UnresolvablePath)?;

        let mut contents = vec![];
        file.read_to_end(&mut contents)
            .await
            .map_err(|_| Error::BadEncoding)?;

        let res = toml::from_str(str::from_utf8(&contents[..]).map_err(|_| Error::BadEncoding)?)
            .map_err(|e| Error::Toml(e))?;

        Ok(res)
    }

    /// Validate that necessary options are set before launching.
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            port: 8000,
            database_path: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_allows_overriding_defaults() {
        let _opts = Config::new();
    }

    #[tokio::test]
    async fn it_loads_file_from_disk() {}
}
