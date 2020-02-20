//! rosu is a rust api wrapper for the game [osu!](https://osu.ppy.sh/home)
//!
//! View the [examples] on how to create requests and fetch their data.
//!
//! The naive internal ratelimiter limits the amount of requests to
//! roughly 10 requests per second.

#![deny(rust_2018_idioms)]

/// Contains the client and the request logic
pub mod backend;
/// Contains all osu! related data structs
pub mod models;

mod util;

#[macro_use]
extern crate log;
#[macro_use]
extern crate num_derive;

pub use backend::{Osu, OsuError, OsuResult};
