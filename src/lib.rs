//! rosu is a rust wrapper for [osu!](https://osu.ppy.sh/home).
//!
//! The wrapper provides access to the [osu!api](https://github.com/ppy/osu-api/wiki)'s
//! beatmap, user, score, user-best, user-recent, and match endpoints.
//!
//! **Note:** Only the osu!api v1 is supported. If you want to use v2, check out [rosu-v2](https://github.com/MaxOhn/rosu-v2).
//!
//! An API key can be generated [here](https://github.com/ppy/osu-api/wiki#requesting-access).
//!
//! Simply initialize an [`Osu`](crate::Osu) client with the api key, call its methods and await the result.
//!
//! ## Examples
//!
//! ```no_run
//! use rosu::{
//!     model::*,
//!     Osu, OsuResult,
//! };
//! use time::OffsetDateTime;
//!
//! #[tokio::main]
//! async fn main() -> OsuResult<()> {
//!     // Initialize the client
//!     # let osu: Osu = {
//!     # /*
//!     let osu = Osu::new("osu_api_key");
//!     # */
//!     # panic!()
//!     # };
//!
//!     // --- Retrieving top scores ---
//!
//!     // Accumulate all important arguments for the request
//!     let request = osu.top_scores("Badewanne3")
//!         .mode(GameMode::Mania)
//!         .limit(4);
//!     // Await the request
//!     let mut scores: Vec<Score> = request.await?;
//!     match scores.pop() {
//!         Some(score) => {
//!             // Retrieve user of the score
//!             let user = score.get_user(&osu).mode(GameMode::Osu).await?;
//!             // ...
//!         }
//!         None => println!("No top scores found"),
//!     }
//!
//!     // --- Retrieving beatmaps ---
//!
//!     let request = osu.beatmaps()
//!         .mode(GameMode::Mania)
//!         .limit(3)
//!         .since(OffsetDateTime::from_unix_timestamp(1542150088).unwrap())
//!         .mapset_id(945496);
//!     let mut maps: Vec<Beatmap> = request.await?;
//!     if let Some(map) = maps.pop() {
//!         let leaderboard: Vec<Score> = map.get_global_leaderboard(&osu).limit(13).await?;
//!         // ...
//!     }
//!
//!     // --- Retrieving user ---
//!
//!     let user: Option<User> = osu.user("Badewanne3").await?;
//!     // ...
//!
//!     // --- Retrieving match ---
//!
//!     let osu_match: Match = osu.osu_match(58494587).await?;
//!     // ...
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Features
//!
//! | Flag        | Description                                            | deps                                                |
//! | ----------- | ------------------------------------------------------ | --------------------------------------------------- |
//! | `serialize` | Provides serialization for all structs in the `models` dir | [serde-repr](https://github.com/dtolnay/serde-repr) |
//! | `metrics`   | Make the client count each request type and enable a method on the client to get a `prometheus::IntCounterVec` | [prometheus](https://github.com/tikv/rust-prometheus)
//!

#![deny(clippy::all, nonstandard_style, rust_2018_idioms, unused, warnings)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate bitflags;

/// Contains the Osu client
mod client;
/// Contains any kind of OsuError that can occur
mod error;
#[cfg(feature = "metrics")]
/// Contains the struct that keeps track of the amount of requests the client does
pub(crate) mod metrics;
/// Contains structs that are parsed from the osu!api
pub mod model;
/// Re-exporting a bunch of things
pub mod prelude;
/// Contains the ratelimiter for the client
pub(crate) mod ratelimit;
/// Contains the Future structs that request the data
pub mod request;
/// Contains the Route enum, responsible for generating the url
mod routing;
/// Contains methods and implementations to (de)serialize structs
pub(crate) mod serde;

pub use error::{OsuError, OsuResult};

pub use client::{Osu, OsuBuilder};
