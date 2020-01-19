//! rosu is a rust api wrapper for the game [osu!](https://osu.ppy.sh/home)
//!
//! View the [examples] on how to create requests and fetch their data.
//!
//! The Osu struct needs to be mutable since it internally updates
//! a cache of URLs and their responses, currently only used for beatmaps
//! since caching users and scores would not make sense.
//!
//! The naive internal ratelimiter limits the amount of requests to
//! roughly 10 requests per second.

#![deny(rust_2018_idioms)]

/// Contains the client and the request logic
pub mod backend;
/// Contains all osu! related data structures
pub mod models;
/// Contains utilities such as the ratelimiter
mod util;

#[macro_use]
extern crate log;
#[macro_use]
extern crate num_derive;

pub use backend::{Osu, OsuError};

#[cfg(test)]
mod tests {
    use super::{
        backend::{requests::*, Osu},
        models::*,
    };
    use chrono::DateTime;
    use std::env;
    use tokio::runtime::Runtime;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    #[ignore]
    fn get_user() {
        init();
        let mut rt = Runtime::new().unwrap();
        rt.block_on(async move {
            kankyo::load().expect("Could not read .env file");
            let osu_key = env::var("OSU_TOKEN").expect("Could not find env variable 'OSU_TOKEN'");
            let osu = Osu::new(osu_key);
            let request = UserRequest::with_username("Badewanne3");
            let user: User = osu
                .prepare_request(request)
                .queue()
                .await
                .unwrap()
                .pop()
                .unwrap();
            let join_date = DateTime::parse_from_rfc3339("2012-12-24T19:48:09-00:00").unwrap();
            assert_eq!(user.join_date, join_date);
        });
    }

    #[test]
    #[ignore]
    fn get_maps() {
        init();
        let mut rt = Runtime::new().unwrap();
        rt.block_on(async move {
            kankyo::load().expect("Could not read .env file");
            let osu_key = env::var("OSU_TOKEN").expect("Could not find env variable 'OSU_TOKEN'");
            let osu = Osu::new(osu_key);
            let request = BeatmapRequest::new().mapset_id(767387);
            let maps: Vec<Beatmap> = osu.prepare_request(request).queue().await.unwrap();
            assert_eq!(maps.len(), 2);
            let map = maps.get(0).unwrap();
            assert_eq!(map.creator, "Mijn Aim Zuigt");
        });
    }

    #[test]
    #[ignore]
    fn get_score() {
        init();
        let mut rt = Runtime::new().unwrap();
        rt.block_on(async move {
            kankyo::load().expect("Could not read .env file");
            let osu_key = env::var("OSU_TOKEN").expect("Could not find env variable 'OSU_TOKEN'");
            let osu = Osu::new(osu_key);
            let request = ScoreRequest::with_map_id(905576)
                .username("spamblock")
                .mode(GameMode::MNA);
            let scores: Vec<Score> = osu.prepare_request(request).queue().await.unwrap();
            assert_eq!(scores.len(), 4);
            let score = scores.get(2).unwrap();
            assert_eq!(score.max_combo, 1293);
        })
    }

    #[test]
    #[ignore]
    fn get_best() {
        init();
        let mut rt = Runtime::new().unwrap();
        rt.block_on(async move {
            kankyo::load().expect("Could not read .env file");
            let osu_key = env::var("OSU_TOKEN").expect("Could not find env variable 'OSU_TOKEN'");
            let osu = Osu::new(osu_key);
            let request = UserBestRequest::with_username("Badewanne3")
                .mode(GameMode::TKO)
                .limit(8);
            let scores: Vec<Score> = osu.prepare_request(request).queue().await.unwrap();
            assert_eq!(scores.len(), 8);
            let score = scores.get(6).unwrap();
            assert_eq!(score.count100, 22);
        })
    }
}
