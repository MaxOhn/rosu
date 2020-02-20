extern crate rosu;

use chrono::DateTime;
use rosu::{
    backend::{
        requests::{BeatmapArgs, MatchArgs, OsuArgs, ScoreArgs, UserArgs, UserBestArgs},
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
        let mut users: Vec<User> = osu.create_request(request).queue().await.unwrap();
        let user = users.pop().unwrap();
        let join_date = DateTime::parse_from_rfc3339("2012-12-24T19:48:09-00:00").unwrap();
        assert_eq!(user.join_date, join_date);
        let best = user.get_top_scores(&osu, 8, GameMode::STD).await.unwrap();
        assert_eq!(best.len(), 8);
    });
}

#[test]
fn get_maps() {
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async move {
        let osu_key = init();
        let osu = Osu::new(osu_key);
        let args = BeatmapArgs::new().mapset_id(1086483);
        let request = OsuArgs::Beatmaps(args);
        let maps: Vec<Beatmap> = osu.create_request(request).queue().await.unwrap();
        assert_eq!(maps.len(), 2);
        let map = maps.get(0).unwrap();
        assert_eq!(map.creator, "Mao");
        let leaderboard = map.get_global_leaderboard(&osu, 7).await.unwrap();
        assert_eq!(leaderboard.len(), 7);
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
        let user = score.get_user(&osu, GameMode::MNA).await.unwrap();
        assert_eq!(user.username, "spamblock");
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

#[test]
fn get_match() {
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async move {
        let osu_key = init();
        let osu = Osu::new(osu_key);
        let args = MatchArgs::with_match_id(58494587);
        let request = OsuArgs::Match(args);
        let osu_match: Match = osu.create_request(request).queue().await.unwrap();
        assert_eq!(osu_match.match_id, 58494587);
        assert_eq!(osu_match.games.len(), 8);
        for game in osu_match.games.iter() {
            assert_eq!(game.scores.len(), 4);
        }
    })
}
