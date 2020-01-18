use chrono::{offset::TimeZone, DateTime, Utc};
use rosu::{
    backend::{
        requests::{BeatmapRequest, OsuRequest, UserBestRequest},
        Osu, OsuError, LazilyLoaded,
    },
    models::{Beatmap, GameMod, GameMode, Score, User},
};
use tokio;

#[tokio::main]
async fn main() -> Result<(), OsuError> {
    // Initialize the client
    let osu = Osu::new("osu_api_key".to_owned());

    // --- Retrieving top scores ---

    // Create a basic top scores request of a user
    let best_request = UserBestRequest::with_username("Badewanne3")
        .mode(GameMode::MNA)
        .limit(4);
    // Let the client finish up the request
    let osu_request: OsuRequest<Score> = osu.prepare_request(best_request);
    // Asynchronously queue the request and retrieve the data
    let mut scores: Vec<Score> = osu_request.queue().await?;
    match scores.pop() {
        Some(score) => {
            // Score struct contains LazilyLoaded fields
            let lazy_user: LazilyLoaded<User> = score.user;
            // Retrieve data for those fields
            let user = lazy_user.get().await?;
            
            // ...
        },
        None => println!("No best score found"),
    }

    // --- Retrieving a beatmap ---

    let since_date: DateTime<Utc> = Utc
        .datetime_from_str("2018-11-13 23:01:28", "%Y-%m-%d %H:%M:%S")
        .unwrap();
    let map_request = BeatmapRequest::new()
        .mode(GameMode::MNA)
        .limit(17)
        .mods(&[GameMod::Key4, GameMod::Hidden])
        .since(since_date)
        .mapset_id(945496);
    let maps: Vec<Beatmap> = osu.prepare_request(map_request).queue().await?;

    // ...

    Ok(())
}
