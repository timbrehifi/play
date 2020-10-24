// Copyright 2020 Nicholas Young. All rights reserved.
//
// Use of this source code is governed by the Mozilla Public License,
// version 2.0, which can be found in the included `LICENSE` file,
// or at https://www.mozilla.org/en-US/MPL/2.0.

use play::Play;
use std::{
    error::Error,
    path::{Path, PathBuf},
};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    let opt = CLI::from_args();

    let mut server = Play::new();
    if let Some(config) = opt.config() {
        server.load_config(config).await?;
    } else {
        server.load_env()?;
    }

    server.run().await?;
    Ok(())
}

/// ultralight networked media server 🔊
#[derive(StructOpt)]
struct CLI {
    /// Optional path to a configuration file on disk
    #[structopt(short, long)]
    config: Option<PathBuf>,
}

impl CLI {
    fn config(&self) -> Option<&Path> {
        if let Some(config) = &self.config {
            return Some(config.as_ref());
        }

        None
    }
}
