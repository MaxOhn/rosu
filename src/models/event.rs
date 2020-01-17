use chrono::{offset::TimeZone, DateTime, Utc};
use serde::{
    de::{Error, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use serde_json::Value as ValEnum;
use std::{fmt, str::FromStr};

/// Event struct for events whithin the `User` struct.
/// Since some events, like acquiring/extending supporter
/// status, do not include map id and mapset id, those
/// fields are whithin an `Option`
#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    html: String,
    beatmap_id: Option<u32>,
    beatmapset_id: Option<u32>,
    date: DateTime<Utc>,
    epic_factor: u32,
}

impl Event {
    pub fn new(
        html: String,
        beatmap_id: Option<u32>,
        beatmapset_id: Option<u32>,
        date: DateTime<Utc>,
        epic_factor: u32,
    ) -> Self {
        Self {
            html,
            beatmap_id,
            beatmapset_id,
            date,
            epic_factor,
        }
    }
}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Html,
            BeatmapId,
            BeatmapsetId,
            Date,
            EpicFactor,
        };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(d: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                        formatter.write_str("`display_html`, `beatmap_id`, `beatmapset_id`, `date`, or `epicfactor`")
                    }

                    fn visit_str<E: Error>(self, value: &str) -> Result<Field, E> {
                        match value {
                            "display_html" => Ok(Field::Html),
                            "beatmap_id" => Ok(Field::BeatmapId),
                            "beatmapset_id" => Ok(Field::BeatmapsetId),
                            "date" => Ok(Field::Date),
                            "epicfactor" => Ok(Field::EpicFactor),
                            _ => Err(Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                d.deserialize_identifier(FieldVisitor)
            }
        }

        struct EventVisitor;

        impl<'de> Visitor<'de> for EventVisitor {
            type Value = Event;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("struct Event")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Event, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut html = None;
                let mut map_id = None;
                let mut mapset_id = None;
                let mut date = None;
                let mut epic_factor = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Html => {
                            if html.is_some() {
                                return Err(Error::duplicate_field("display_html"));
                            }
                            html = Some(map.next_value()?);
                        }
                        Field::BeatmapId => {
                            if map_id.is_some() {
                                return Err(Error::duplicate_field("beatmap_id"));
                            }
                            match map.next_value()? {
                                ValEnum::Null => {}
                                ValEnum::String(s) => {
                                    map_id = Some(u32::from_str(&s).map_err(Error::custom)?)
                                }
                                _ => {
                                    return Err(V::Error::custom(
                                        "Expected string or null, found other type",
                                    ))
                                }
                            }
                        }
                        Field::BeatmapsetId => {
                            if mapset_id.is_some() {
                                return Err(Error::duplicate_field("beatmapset_id"));
                            }
                            match map.next_value()? {
                                ValEnum::Null => {}
                                ValEnum::String(s) => {
                                    mapset_id = Some(u32::from_str(&s).map_err(Error::custom)?)
                                }
                                _ => {
                                    return Err(V::Error::custom(
                                        "Expected string or null, found other type",
                                    ))
                                }
                            }
                        }
                        Field::Date => {
                            if date.is_some() {
                                return Err(Error::duplicate_field("date"));
                            }
                            let s: String = map.next_value()?;
                            date = Some(Utc.datetime_from_str(&s, "%F %T").map_err(Error::custom)?);
                        }
                        Field::EpicFactor => {
                            if epic_factor.is_some() {
                                return Err(Error::duplicate_field("epic_factor"));
                            }
                            epic_factor = Some(map.next_value()?);
                        }
                    }
                }
                let html = html.ok_or_else(|| Error::missing_field("html"))?;
                let date = date.ok_or_else(|| Error::missing_field("date"))?;
                let epic_factor = epic_factor.ok_or_else(|| Error::missing_field("epic_factor"))?;
                let epic_factor = u32::from_str(epic_factor).map_err(Error::custom)?;
                Ok(Event::new(html, map_id, mapset_id, date, epic_factor))
            }
        }

        const FIELDS: &[&str] = &[
            "display_html",
            "beatmap_id",
            "beatmapset_id",
            "date",
            "epicfactor",
        ];
        d.deserialize_struct("Event", FIELDS, EventVisitor)
    }
}
