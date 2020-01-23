use chrono::{offset::TimeZone, DateTime, Utc};
use rosu::{
    Osu, OsuError, 
    backend::{requests::*, LazilyLoaded},
    models::*,
};

#[tokio::main]
async fn main() -> Result<(), OsuError> {
    // Initialize the client
    let osu = Osu::new("osu_api_key".to_owned());

    // --- Retrieving top scores ---

    // Cummulate all important arguments for the request
    let user_args = UserBestArgs::with_username("Badewanne3")
        .mode(GameMode::MNA)
        .limit(4);
    // Put the arguments in the arguments wrapper
    let args = OsuArgs::Best(user_args);
    // Let the client create the request
    let osu_request: OsuRequest<Score> = osu.create_request(args);
    // Asynchronously queue the request and retrieve the data
    let mut scores: Vec<Score> = osu_request.queue().await?;
    match scores.pop() {
        Some(score) => {
            // Score struct contains some LazilyLoaded fields
            let lazy_user: &LazilyLoaded<User> = &score.user;
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
    let args = BeatmapArgs::new()
        .mode(GameMode::MNA)
        .limit(3)
        .mods(&[GameMod::Key4, GameMod::Hidden])
        .since(since_date)
        .mapset_id(945496);
    let maps: Vec<Beatmap> = osu.create_request(OsuArgs::Beatmaps(args)).queue().await?;

    // ...

    Ok(())
}
