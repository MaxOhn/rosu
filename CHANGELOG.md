# v0.6.0 (2023-07-08)

- Breaking
  - Renamed the `GameMode` variants to `Osu`, `Taiko`, `Catch`, and `Mania`
  - Replaced the [`chrono`] dependency with [`time`]. All fields of type `chrono::DateTime<chrono::offset::Utc>` are now of type `time::OffsetDateTime`
  - Removed the `cache` feature and all types & methods that came with it.
  - Renamed the struct `APIError` to `ApiError`
  - The given api key must now implement `Into<Box<str>>` instead of `Into<String>`
  - `OsuBuilder` no longer implements `Default`

- Dependencies
  - Bumped [`prometheus`] from 0.11 to 0.13
  - Added [`thiserror`]

[`chrono`]: https://crates.io/crates/chrono
[`time`]: https://crates.io/crates/time
[`prometheus`]: https://crates.io/crates/prometheus
[`thiserror`]: https://crates.io/crates/prometheus