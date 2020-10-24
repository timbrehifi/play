// Copyright 2020 Nicholas Young. All rights reserved.
//
// Use of this source code is governed by the Mozilla Public License,
// version 2.0, which can be found in the included `LICENSE` file,
// or at https://www.mozilla.org/en-US/MPL/2.0.

use crate::{config::Config, Result};
use std::path::Path;

/// An instance of the Play server, which contains all necessary
/// runtime configuration and can be updated with an optional
/// configuration file loaded from disk.
#[derive(Default)]
pub struct Play {
    config: Config,
}

impl Play {
    /// Create a new instance of the Play service with default
    /// configuration.
    pub fn new() -> Self {
        Play::default()
    }

    /// Populate configuration from current environment variables.
    pub fn load_env(&mut self) -> Result<()> {
        self.config = Config::from_env()?;
        Ok(())
    }

    /// Populate configuration from a path to file on disk.
    pub async fn load_config<P>(&mut self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        self.config = Config::from_file(path).await?;
        Ok(())
    }

    /// Attempt to launch a configured instance of the service.
    pub async fn run(&self) -> Result<()> {
        self.setup()?;
        Ok(())
    }

    // Perform final setup checks, like ensuring configuration is
    // valid before the service launches.
    fn setup(&self) -> Result<()> {
        self.config.validate()?;
        Ok(())
    }
}
