# rosu

rosu is a wrapper of the [osu!api](https://github.com/ppy/osu-api/wiki) written in rust.

Requests use either the `/get_beatmaps`, `/get_user`, `/get_scores`, `/get_user_best`, or `/get_user_recent` endpoints
via the `BeatmapRequest`, `UserRequest`, `ScoreRequest`, `UserBestRequest`, and `UserRecentRequest` structs, respectively.

The client needs to be mutable because it modifies its internal cache by being able to memorize requested url's and their responses.
Its internal ratelimiter limits the amount of requests to the api to about 10 requests per second.

### Example
```rust
use chrono::{offset::TimeZone, DateTime, Utc};
use rosu::{
    backend::{
        requests::{BeatmapRequest, OsuRequest, UserRequest},
        Osu, OsuError,
    },
    models::{Beatmap, GameMod, GameMode, User},
};
use tokio;

#[tokio::main]
async fn main() -> Result<(), OsuError> {
    // Initialize the client
    let mut osu = Osu::new("osu_api_key".to_owned());
    // Create a basic user request
    let user_request = UserRequest::with_username("Badewanne3").mode(GameMode::TKO);
    // Let the client finish up the request
    let osu_request: OsuRequest<User> = osu.prepare_request(user_request);
    // Asynchronously queue the request and retrieve the data
    let mut users: Vec<User> = osu_request.queue().await?;
    let user = users.pop().unwrap();

    // ...

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
```