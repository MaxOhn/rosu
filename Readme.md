# rosu

rosu is a wrapper of the [osu!api](https://github.com/ppy/osu-api/wiki) written in rust.

Requests use either the `/get_beatmaps`, `/get_user`, `/get_scores`, `/get_user_best`, or `/get_user_recent` endpoints
through the `BeatmapRequest`, `UserRequest`, `ScoreRequest`, `UserBestRequest`, and `UserRecentRequest` structs, respectively.

The client consists of an internal cache to memorize requested url's and their responses.
Its internal ratelimiter limits the amount of requests to the api to about 10 requests per second.

### Example
```rust
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
    let args = UserBestArgs::with_username("Badewanne3")
        .mode(GameMode::MNA)
        .limit(4);
    // Put the arguments in the request wrapper
    let best_request = Request::Best(args);
    // Let the client finish up the request
    let osu_request: OsuRequest<Score> = osu.prepare_request(best_request);
    // Asynchronously queue the request and retrieve the data
    let mut scores: Vec<Score> = osu_request.queue().await?;
    match scores.pop() {
        Some(score) => {
            // Score struct contains some LazilyLoaded fields
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
    let args = BeatmapArgs::new()
        .mode(GameMode::MNA)
        .limit(17)
        .mods(&[GameMod::Key4, GameMod::Hidden])
        .since(since_date)
        .mapset_id(945496);
    let map_request = Request::Beatmaps(args);
    let maps: Vec<Beatmap> = osu.prepare_request(map_request).queue().await?;

    // ...

    Ok(())
}
```
