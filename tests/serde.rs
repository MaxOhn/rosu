extern crate rosu;

#[cfg(feature = "serialize")]
use rosu::models::*;

#[cfg(feature = "serialize")]
#[test]
fn serde_score() {
    let mut score = Score::default();
    score.score = 1_000_000;
    score.enabled_mods = GameMods::from_bits(24).unwrap();
    let serialized = serde_json::to_string(&score).unwrap();
    println!("{}", serialized);
    let deserialized = serde_json::from_str(&serialized).unwrap();
    assert_eq!(score, deserialized);
}

#[cfg(feature = "serialize")]
#[test]
fn serde_beatmap() {
    let mut map = Beatmap::default();
    map.rating = 4.2;
    let serialized = serde_json::to_string(&map).unwrap();
    println!("{}", serialized);
    let deserialized = serde_json::from_str(&serialized).unwrap();
    assert_eq!(map, deserialized);
}

#[cfg(feature = "serialize")]
#[test]
fn serde_user() {
    let mut user = User::default();
    user.accuracy = 97.65;
    let serialized = serde_json::to_string(&user).unwrap();
    println!("{}", serialized);
    let deserialized = serde_json::from_str(&serialized).unwrap();
    assert_eq!(user, deserialized);
}
