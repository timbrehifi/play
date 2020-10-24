// Copyright 2020 Nicholas Young. All rights reserved.
//
// Use of this source code is governed by the Mozilla Public License,
// version 2.0, which can be found in the included `LICENSE` file,
// or at https://www.mozilla.org/en-US/MPL/2.0.

//! Play is an ultralight networked media server designed for and by
//! hi-fi enthusiasts.

mod config;
mod play;
mod result;

pub use crate::{play::Play, result::Result};
