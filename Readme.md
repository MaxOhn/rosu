[![crates.io](https://img.shields.io/crates/v/rosu.svg)](https://crates.io/crates/rosu) [![docs](https://docs.rs/rosu/badge.svg)](https://docs.rs/rosu)

# rosu

rosu is a rust wrapper for [osu!](https://osu.ppy.sh/home).

The wrapper provides access to the [osu!api](https://github.com/ppy/osu-api/wiki)'s
beatmap, user, score, user-best, user-recent, and match endpoints.
*Note:* Only v1 of the osu!api is supported.

An API key can be generated [here](https://github.com/ppy/osu-api/wiki#requesting-access).

Simply initialize an [`Osu`](crate::Osu) client with the api key, call any of its `get_*` methods
and await its result.

### Examples

```rust
use chrono::{offset::TimeZone, DateTime, Utc};
use rosu::{
    model::*,
    Osu, OsuResult,
};

#[tokio::main]
async fn main() -> OsuResult<()> {
    // Initialize the client
    # let osu: Osu = {
    # /*
    let osu = Osu::new("osu_api_key");
    # */
    # panic!()
    # };
    // If `cache` feature enabled:
    // let osu = Osu::new("osu_api_key", redis_pool, rosu::OsuCached::User);

    // --- Retrieving top scores ---

    // Accumulate all important arguments for the request
    let request = osu.top_scores("Badewanne3")
        .mode(GameMode::MNA)
        .limit(4);
    // Await the request
    let mut scores: Vec<Score> = request.await?;
    match scores.pop() {
        Some(score) => {
            // Retrieve user of the score
            let user = score.get_user(&osu).mode(GameMode::STD).await?;
            // ...
        }
        None => println!("No top scores found"),
    }

    // --- Retrieving beatmaps ---

    let since_date: DateTime<Utc> = Utc
        .datetime_from_str("2018-11-13 23:01:28", "%Y-%m-%d %H:%M:%S")
        .unwrap();
    let request = osu.beatmaps()
        .mode(GameMode::MNA)
        .limit(3)
        .since(since_date)
        .mapset_id(945496);
    let mut maps: Vec<Beatmap> = request.await?;
    if let Some(map) = maps.pop() {
        let leaderboard: Vec<Score> = map.get_global_leaderboard(&osu).limit(13).await?;
        // ...
    }

    // --- Retrieving user ---

    let user: Option<User> = osu.user("Badewanne3").await?;
    // ...

    // --- Retrieving match ---

    let osu_match: Match = osu.osu_match(58494587).await?;
    // ...

    Ok(())
}
```

#### Features

| Flag        | Description                                            | deps                                                |
| ----------- | ------------------------------------------------------ | --------------------------------------------------- |
| `serialize` | Provides serialization for all structs in the `models` dir | [serde-repr](https://github.com/dtolnay/serde-repr) |
| `metrics`   | Make the client count each request type and enable a method on the client to get a `prometheus::IntCounterVec` | [prometheus](https://github.com/tikv/rust-prometheus)
| `cache`     | Cache API results through a redis connection for a given duration | [darkredis](https://github.com/Bunogi/darkredis), `serialize` |
