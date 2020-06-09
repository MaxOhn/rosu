extern crate rosu;

use chrono::DateTime;
use rosu::{
    backend::{
        requests::{BeatmapRequest, BestRequest, MatchRequest, ScoreRequest, UserRequest},
        Osu,
    },
    models::*,
};
use std::env;
use tokio::runtime::Runtime;

fn init() -> String {
    let _ = env_logger::builder().is_test(true).try_init();
    kankyo::load(true).expect("Could not read .env file");
    env::var("OSU_TOKEN").expect("Could not find env variable 'OSU_TOKEN'")
}

#[test]
fn get_user() {
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async move {
        let osu_key = init();
        let osu = Osu::new(osu_key);
        let user = UserRequest::with_username("Badewanne3")
            .queue_single(&osu)
            .await
            .unwrap()
            .unwrap();
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
        let maps = BeatmapRequest::new()
            .mapset_id(1086483)
            .queue(&osu)
            .await
            .unwrap();
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
        let scores = ScoreRequest::with_map_id(905576)
            .username("spamblock")
            .mode(GameMode::MNA)
            .queue(&osu)
            .await
            .unwrap();
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
        let scores = BestRequest::with_username("Badewanne3")
            .mode(GameMode::TKO)
            .limit(8)
            .queue(&osu)
            .await
            .unwrap();
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
        let osu_match = MatchRequest::with_match_id(58494587)
            .queue_single(&osu)
            .await
            .unwrap();
        assert_eq!(osu_match.match_id, 58494587);
        assert_eq!(osu_match.games.len(), 8);
        for game in osu_match.games.iter() {
            assert_eq!(game.scores.len(), 4);
        }
    })
}
