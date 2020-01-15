#![allow(dead_code)]

pub mod backend;
pub mod models;
pub mod util;

#[macro_use]
extern crate log;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate num_derive;

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use super::{
        backend::{requests::*, Osu, OsuError},
        models::*,
        util::*,
    };
    use chrono::{DateTime, Utc};
    use std::{env, cmp};
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
            let mut osu = Osu::new(osu_key);
            let request = UserRequest::with_username("Badewanne3".to_owned());
            let user = osu
                .get_users(request)
                .unwrap()
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
            let mut osu = Osu::new(osu_key);
            let request = BeatmapRequest::new().mapset_id(767387);
            let maps = osu.get_maps(request).unwrap().queue().await.unwrap();
            assert_eq!(maps.len(), 2);
            let map = maps.get(0).unwrap();
            assert_eq!(map.creator, "Mijn Aim Zuigt");
        });
    }

    #[test]
    fn get_score() {
        init();
        let mut rt = Runtime::new().unwrap();
        rt.block_on(async move {
            kankyo::load().expect("Could not read .env file");
            let osu_key = env::var("OSU_TOKEN").expect("Could not find env variable 'OSU_TOKEN'");
            let mut osu = Osu::new(osu_key);
            let request = ScoreRequest::with_map_id(905576)
                .username("spamblock".to_owned())
                .mode(GameMode::MNA);
            let mut scores: Vec<Score> = osu.get_scores(request).unwrap().queue().await.unwrap();
            assert_eq!(scores.len(), 4);
            let score = scores.get(2).unwrap();
            assert_eq!(score.max_combo, 1293);
        })
    }

    #[test]
    #[ignore]
    fn test_ratelimiter() {
        let start = Utc::now().timestamp_millis();
        let mut ratelimiter = RateLimiter::new(500, 7);
        let mut counter = 0;
        while counter < 53 {
            ratelimiter.wait_access();
            counter += 1;
        }
        let end = Utc::now().timestamp_millis();
        let diff = end - start;
        // Make sure the limiter actually waits to grant access but doesn't take too long
        assert!(diff < 5000 && diff > 3500);
    }
}
