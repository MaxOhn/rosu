//! rosu is a rust wrapper for [osu!](https://osu.ppy.sh/home).
//!
//! The wrapper provides access to the [osu!api](https://github.com/ppy/osu-api/wiki)'s
//! beatmap, user, score, user-best, user-recent, and match endpoints.
//! *Note:* Only v1 of the osu!api is supported.
//!
//! An API key can be generated [here](https://github.com/ppy/osu-api/wiki#requesting-access).
//!
//! Simply initialize an [`Osu`](crate::Osu) client with the api key, call any of its `get_*` methods
//! and await its result.
//!
//! ## Examples
//!
//! ```no_run
//! use chrono::{offset::TimeZone, DateTime, Utc};
//! use rosu::{
//!     model::*,
//!     Osu, OsuError,
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), OsuError> {
//!     // Initialize the client
//!     # let osu: Osu = {
//!     # /*
//!     let osu = Osu::new("osu_api_key");
//!     # */
//!     # panic!()
//!     # };
//!     // If `cache` feature enabled:
//!     // let osu = Osu::new("osu_api_key", redis_pool, rosu::OsuCached::User);
//!
//!     // --- Retrieving top scores ---
//!
//!     // Accumulate all important arguments for the request
//!     let request = osu.top_scores("Badewanne3")
//!         .mode(GameMode::MNA)
//!         .limit(4);
//!     // Await the request
//!     let mut scores: Vec<Score> = request.await?;
//!     match scores.pop() {
//!         Some(score) => {
//!             // Retrieve user of the score
//!             let user = score.get_user(&osu).mode(GameMode::STD).await?;
//!             // ...
//!         }
//!         None => println!("No top scores found"),
//!     }
//!
//!     // --- Retrieving beatmaps ---
//!
//!     let since_date: DateTime<Utc> = Utc
//!         .datetime_from_str("2018-11-13 23:01:28", "%Y-%m-%d %H:%M:%S")
//!         .unwrap();
//!     let request = osu.beatmaps()
//!         .mode(GameMode::MNA)
//!         .limit(3)
//!         .since(since_date)
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
//! | `cache`     | Cache API results through a redis connection for a given duration | [darkredis](https://github.com/Bunogi/darkredis), `serialize` |
//!
//! ### Error handling
//! [`OsuError`](crate::OsuError)s are nested through their source errors. To read them, one can use a small unwind macro such as:
//!
//! ```no_run
//! #[macro_export]
//! macro_rules! unwind_error {
//!     ($log:ident, $err:ident, $($arg:tt)+) => {
//!         {
//!             $log!($($arg)+, $err);
//!             let mut err: &dyn ::std::error::Error = &$err;
//!             while let Some(source) = err.source() {
//!                 $log!("  - caused by: {}", source);
//!                 err = source;
//!             }
//!         }
//!     };
//! }
//!
//! use rosu::{Osu, GameMode};
//!
//! #[tokio::main]
//! async main fn() {
//!     # let osu: Osu = {
//!     # /*
//!     let osu = Osu::new("osu_api_key");
//!     # */
//!     # panic!()
//!     # };
//!     let mode = GameMode::STD;
//!     if let Err(why) = osu.user("badewanne3").mode(mode).await {
//!         unwind_error!(println, why, "Error while retrieving user for mode {}", mode);
//!     }
//! }
//! ```

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

#[cfg(feature = "cache")]
pub use client::OsuCached;
