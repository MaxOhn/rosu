//! rosu is a wrapper for [osu!](https://osu.ppy.sh/home) written in rust.
//!
//! The wrapper provides access to the [osu!api](https://github.com/ppy/osu-api/wiki)'s
//! beatmap, user, score, user-best, user-recent, and match endpoints
//! with a request struct for each endpoint, e.g. [`BestRequest`] for the user-best endpoint.
//!
//! Simply initialize an [`Osu`] client, formulate a request such as a [`UserRequest`],
//! and then retrieve the data by calling the request's `queue` method with a reference to the
//! client as argument.
//!
//! [`UserRequest`]: backend/requests/struct.UserRequest.html
//! [`BestRequest`]: backend/requests/struct.BestRequest.html
//! [`Osu`]: backend/struct.Osu.html
//!
//! ## Examples
//!
//! ```no_run
//! use chrono::{offset::TimeZone, DateTime, Utc};
//! use rosu::{
//!     backend::{BeatmapRequest, BestRequest, MatchRequest, UserRequest},
//!     models::*,
//!     Osu, OsuError,
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), OsuError> {
//!     // Initialize the client
//!     let osu = Osu::new("osu_api_key".to_owned());
//!
//!     // --- Retrieving top scores ---
//!
//!     // Accumulate all important arguments for the request
//!     let request = BestRequest::with_username("Badewanne3")
//!         .mode(GameMode::MNA)
//!         .limit(4);
//!     // Asynchronously send the request through the osu client
//!     let mut scores: Vec<Score> = request.queue(&osu).await?;
//!     match scores.pop() {
//!         Some(score) => {
//!             // Retrieve user of the score
//!             let user = score.get_user(&osu, GameMode::STD).await?;
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
//!     let request = BeatmapRequest::new()
//!         .mode(GameMode::MNA)
//!         .limit(3)
//!         .since(since_date)
//!         .mapset_id(945496);
//!     let mut maps: Vec<Beatmap> = request.queue(&osu).await?;
//!     if let Some(map) = maps.pop() {
//!         let leaderboard: Vec<Score> = map.get_global_leaderboard(&osu, 13).await?;
//!         // ...
//!     }
//!
//!     // --- Retrieving user ---
//!
//!     let user = UserRequest::with_username("Badewanne3")
//!         .queue_single(&osu)
//!         .await?;
//!     // ...
//!
//!     // --- Retrieving match ---
//!
//!     let osu_match = MatchRequest::with_match_id(58494587)
//!         .queue_single(&osu)
//!         .await?;
//!
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
//! | `serialize` | Provides serialization for all structs in `models` dir | [serde-repr](https://github.com/dtolnay/serde-repr) |
//!

#[macro_use]
extern crate log;
#[macro_use]
extern crate bitflags;

/// Contains the client and the request logic
pub mod backend;
/// Contains all osu! related data structs
pub mod models;
/// Contains method to (de)serialize structs
pub(crate) mod serde;

pub use backend::{Osu, OsuError, OsuResult};
