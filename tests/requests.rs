extern crate rosu;

use chrono::DateTime;
use rosu::{
    backend::{
        requests::{BeatmapArgs, OsuArgs, ScoreArgs, UserArgs, UserBestArgs},
        Osu,
    },
    models::*,
};
use std::env;
use tokio::runtime::Runtime;

fn init() -> String {
    let _ = env_logger::builder().is_test(true).try_init();
    kankyo::load().expect("Could not read .env file");
    env::var("OSU_TOKEN").expect("Could not find env variable 'OSU_TOKEN'")
}

#[test]
fn get_user() {
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async move {
        let osu_key = init();
        let osu = Osu::new(osu_key);
        let args = UserArgs::with_username("Badewanne3");
        let request = OsuArgs::Users(args);
        let user: User = osu
            .create_request(request)
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
fn get_maps() {
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async move {
        let osu_key = init();
        let osu = Osu::new(osu_key);
        let args = BeatmapArgs::new().mapset_id(767387);
        let request = OsuArgs::Beatmaps(args);
        let maps: Vec<Beatmap> = osu.create_request(request).queue().await.unwrap();
        assert_eq!(maps.len(), 2);
        let map = maps.get(0).unwrap();
        assert_eq!(map.creator, "Mijn Aim Zuigt");
    });
}

#[test]
fn get_score() {
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async move {
        let osu_key = init();
        let osu = Osu::new(osu_key);
        let args = ScoreArgs::with_map_id(905576)
            .username("spamblock")
            .mode(GameMode::MNA);
        let request = OsuArgs::Scores(args);
        let scores: Vec<Score> = osu.create_request(request).queue().await.unwrap();
        assert_eq!(scores.len(), 4);
        let score = scores.get(2).unwrap();
        assert_eq!(score.max_combo, 1293);
    })
}

#[test]
fn get_best() {
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async move {
        let osu_key = init();
        let osu = Osu::new(osu_key);
        let args = UserBestArgs::with_username("Badewanne3")
            .mode(GameMode::TKO)
            .limit(8);
        let request = OsuArgs::Best(args);
        let scores: Vec<Score> = osu.create_request(request).queue().await.unwrap();
        assert_eq!(scores.len(), 8);
        let score = scores.get(6).unwrap();
        assert_eq!(score.count100, 22);
    })
}