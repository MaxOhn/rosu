use chrono::{offset::TimeZone, DateTime, Utc};
use rosu::{
    backend::{BeatmapRequest, BestRequest, MatchRequest, UserRequest},
    models::*,
    Osu, OsuError,
};

#[tokio::main]
async fn main() -> Result<(), OsuError> {
    // Initialize the client
    let osu = Osu::new("osu_api_key");

    // --- Retrieving top scores ---

    // Accumulate all important arguments for the request
    let request = BestRequest::with_username("Badewanne3")
        .mode(GameMode::MNA)
        .limit(4);
    // Asynchronously send the request through the osu client
    let mut scores: Vec<Score> = request.queue(&osu).await?;
    match scores.pop() {
        Some(score) => {
            // Retrieve user of the score
            let user = score.get_user(&osu, GameMode::STD).await?;
            // ...
        }
        None => println!("No top scores found"),
    }

    // --- Retrieving beatmaps ---

    let since_date: DateTime<Utc> = Utc
        .datetime_from_str("2018-11-13 23:01:28", "%Y-%m-%d %H:%M:%S")
        .unwrap();
    let request = BeatmapRequest::new()
        .mode(GameMode::MNA)
        .limit(3)
        .mods(&GameMods::new(vec![GameMod::Key4, GameMod::Hidden]))
        .since(since_date)
        .mapset_id(945496);
    let mut maps: Vec<Beatmap> = request.queue(&osu).await?;
    if let Some(map) = maps.pop() {
        let leaderboard: Vec<Score> = map.get_global_leaderboard(&osu, 13).await?;
        // ...
    }

    // --- Retrieving user ---

    let user: User = UserRequest::with_username("Badewanne3")
        .queue_single(&osu)
        .await?
        .expect("User was not found");
    // ...

    // --- Retrieving match ---

    let osu_match: Match = MatchRequest::with_match_id(58494587)
        .queue_single(&osu)
        .await?;

    // ...

    Ok(())
}
