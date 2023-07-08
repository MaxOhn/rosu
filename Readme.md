[![crates.io](https://img.shields.io/crates/v/rosu.svg)](https://crates.io/crates/rosu) [![docs](https://docs.rs/rosu/badge.svg)](https://docs.rs/rosu)

# rosu

rosu is a rust wrapper for the [osu!api **v1**](https://github.com/ppy/osu-api/wiki) .

The wrapper provides access to the beatmap, user, score, user-best, user-recent, and match endpoints.

**Note:** Only the osu!api v1 is supported. If you want to use v2, check out [rosu-v2](https://github.com/MaxOhn/rosu-v2).

An API key can be generated [here](https://github.com/ppy/osu-api/wiki#requesting-access).

## Examples
```rust
use rosu::{
    model::*,
    Osu, OsuResult,
};
use time::OffsetDateTime;

#[tokio::main]
async fn main() -> OsuResult<()> {
    // Initialize the client
    let osu = Osu::new("osu_api_key");

    // --- Retrieving top scores ---
    let mut scores = osu.top_scores("Badewanne3")
        .mode(GameMode::Mania)
        .limit(4)
        .await?;
    match scores.pop() {
        Some(score) => {
            // Retrieve user of the score
            let user = score.get_user(&osu).mode(GameMode::Osu).await?;
            // ...
        }
        None => println!("No top scores found"),
    }

    // --- Retrieving beatmaps ---
    let mut maps = osu.beatmaps()
        .mode(GameMode::Mania)
        .limit(3)
        .since(OffsetDateTime::from_unix_timestamp(1542150088).unwrap())
        .mapset_id(945496)
        .await?;
    if let Some(map) = maps.pop() {
        let leaderboard: Vec<Score> = map
            .get_global_leaderboard(&osu)
            .limit(13)
            .await?;
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
### Features
| Flag        | Description                                            | deps                                                |
| ----------- | ------------------------------------------------------ | --------------------------------------------------- |
| `serialize` | Provides serialization for all types in the `model` module | [serde-repr](https://github.com/dtolnay/serde-repr) |
| `metrics`   | Make the client count each request type and enable a method on the client to get a `prometheus::IntCounterVec` | [prometheus](https://github.com/tikv/rust-prometheus)
