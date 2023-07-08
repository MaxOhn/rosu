extern crate rosu;

use rosu::{model::*, Osu, OsuError};
use std::env;
use time::OffsetDateTime;

fn init() -> Osu {
    let _ = env_logger::builder().is_test(true).try_init();
    dotenvy::dotenv().unwrap();
    let api_key = env::var("OSU_TOKEN").unwrap();
    make_osu(api_key)
}

fn make_osu(api_key: String) -> Osu {
    Osu::new(api_key)
}

#[tokio::test]
async fn invalid_api_key() {
    let osu = Osu::new("invalid_api_key");
    let user_error = osu.user(0).await;
    assert!(matches!(user_error, Err(OsuError::Response { .. })));
}

#[tokio::test]
async fn get_user() {
    let osu = init();

    let user = osu
        .user("muse dash")
        .mode(GameMode::Mania)
        .await
        .unwrap()
        .unwrap();
    let join_date = OffsetDateTime::from_unix_timestamp(1_546_498_888).unwrap();
    assert_eq!(user.join_date, join_date);
    let best = user
        .get_top_scores(&osu)
        .limit(8)
        .mode(GameMode::Mania)
        .await
        .unwrap();
    assert_eq!(best.len(), 8);

    #[cfg(feature = "metrics")]
    {
        use prometheus::core::Collector;
        for metric in osu.metrics().collect()[0].get_metric() {
            let name = metric.get_label()[0].get_value();
            let value = metric.get_counter().get_value();
            if ["TopScores", "Users"].contains(&name) {
                assert_eq!(value as i32, 1);
            } else {
                assert_eq!(value as i32, 0);
            }
        }
    }

    #[cfg(feature = "serialize")]
    {
        let serialization = serde_json::to_string(&user).unwrap();
        let deserialization = serde_json::from_str(&serialization).unwrap();
        assert_eq!(user, deserialization);
    }
}

#[tokio::test]
async fn get_maps() {
    let osu = init();
    let maps = osu.beatmaps().mapset_id(1086483).await.unwrap();
    assert_eq!(maps.len(), 2);
    let map = maps.get(0).unwrap();
    assert_eq!(map.creator, "Mao");
    let leaderboard = map.get_global_leaderboard(&osu).limit(7).await.unwrap();
    assert_eq!(leaderboard.len(), 7);

    #[cfg(feature = "serialize")]
    {
        let serialization = serde_json::to_string(&maps).unwrap();
        let deserialization: Vec<Beatmap> = serde_json::from_str(&serialization).unwrap();
        assert_eq!(maps, deserialization);
    }

    let maps = osu.beatmaps().limit(100).await.unwrap();
    assert_eq!(maps.len(), 100);
}

#[tokio::test]
async fn get_score() {
    let osu = init();
    let scores = osu
        .scores(905576)
        .user("spamblock")
        .mode(GameMode::Mania)
        .await
        .unwrap();
    assert_eq!(scores.len(), 4);
    let score = scores.get(2).unwrap();
    assert_eq!(score.max_combo, 1293);
    let user = score
        .get_user(&osu)
        .mode(GameMode::Mania)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(user.username, "spamblock");

    #[cfg(feature = "serialize")]
    {
        let serialization = serde_json::to_string(&scores).unwrap();
        let deserialization: Vec<Score> = serde_json::from_str(&serialization).unwrap();
        assert_eq!(scores, deserialization);
    }
}

#[tokio::test]
async fn get_best() {
    let osu = init();
    let scores = osu
        .top_scores("Badewanne3")
        .mode(GameMode::Taiko)
        .limit(9)
        .limit(8)
        .await
        .unwrap();
    assert_eq!(scores.len(), 8);
    let score = scores.get(6).unwrap();
    assert_eq!(score.count100, 31);

    #[cfg(feature = "serialize")]
    {
        let serialization = serde_json::to_string(&scores).unwrap();
        let deserialization: Vec<Score> = serde_json::from_str(&serialization).unwrap();
        assert_eq!(scores, deserialization);
    }
}

#[tokio::test]
async fn get_recent() {
    let osu = init();
    let _scores = osu.recent_scores("mornis").limit(1).await.unwrap();

    #[cfg(feature = "serialize")]
    {
        let serialization = serde_json::to_string(&_scores).unwrap();
        let deserialization: Vec<Score> = serde_json::from_str(&serialization).unwrap();
        assert_eq!(_scores, deserialization);
    }
}

#[tokio::test]
async fn get_match() {
    let osu = init();
    let osu_match = osu.osu_match(58494587).await.unwrap();
    assert_eq!(osu_match.match_id, 58494587);
    assert_eq!(osu_match.games.len(), 8);
    for game in osu_match.games.iter() {
        assert_eq!(game.scores.len(), 4);
    }
    let match_err = osu.osu_match(68778237).await;
    assert!(matches!(match_err, Err(OsuError::InvalidMultiplayerMatch)));

    #[cfg(feature = "serialize")]
    {
        let serialization = serde_json::to_string(&osu_match).unwrap();
        let deserialization = serde_json::from_str(&serialization).unwrap();
        assert_eq!(osu_match, deserialization);
    }
}
