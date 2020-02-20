# rosu

rosu is a wrapper of the [osu!api](https://github.com/ppy/osu-api/wiki) written in rust.

The wrapper provides access to osu!'s beatmap, user, score, user-best, user-recent, and match endpoints with the help of the argument structs `BeatmapArgs`, `UserArgs`, `ScoreArgs`, `UserBestArgs`, `UserRecentArgs`, and `MatchArgs`, respectively.
Creating those argument structs, then wrapping them into the `OsuArgs` enum and finally calling `Osu::create_request` with the wrapped arguments as parameter will provide an `OsuRequest` that is ready to be sent via `OsuRequest::queue` to retrieve the parsed data.

The clients internal ratelimiter limits the amount of requests to the api to about 10 requests per second.

### Example
```rust
use chrono::{offset::TimeZone, DateTime, Utc};
use rosu::{backend::requests::*, models::*, Osu, OsuError};

#[tokio::main]
async fn main() -> Result<(), OsuError> {
    // Initialize the client
    let osu = Osu::new("osu_api_key");

    // --- Retrieving top scores ---

    // Cummulate all important arguments for the request
    let best_args = UserBestArgs::with_username("Badewanne3")
        .mode(GameMode::MNA)
        .limit(4);
    // Put the arguments in the arguments wrapper
    let args = OsuArgs::Best(best_args);
    // Let the client create the request
    // Careful: Except for the retrieval of osu matches, all responses
    // are going to be a Vec<...>
    let osu_request: OsuRequest<Vec<Score>> = osu.create_request(args);
    // Asynchronously queue the request and retrieve the data
    let mut scores: Vec<Score> = osu_request.queue().await?;
    match scores.pop() {
        Some(score) => {
            // Retrieve user of the score
            let user = score.get_user(&osu, GameMode::STD).await?;
            // ...
        }
        None => println!("No best score found"),
    }

    // --- Retrieving beatmaps ---

    let since_date: DateTime<Utc> = Utc
        .datetime_from_str("2018-11-13 23:01:28", "%Y-%m-%d %H:%M:%S")
        .unwrap();
    let args = BeatmapArgs::new()
        .mode(GameMode::MNA)
        .limit(3)
        .mods(&GameMods::new(vec![GameMod::Key4, GameMod::Hidden]))
        .since(since_date)
        .mapset_id(945496);
    let mut maps: Vec<Beatmap> = osu.create_request(OsuArgs::Beatmaps(args)).queue().await?;
    if let Some(map) = maps.pop() {
        let leaderboard: Vec<Score> = map.get_global_leaderboard(&osu, 13).await?;
        // ...
    }

    // ...

    Ok(())
}
```
