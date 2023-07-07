use time::format_description::{
    modifier::{Day, Hour, Minute, Month, Second, Year},
    Component, FormatItem,
};

const DATE_FORMAT: &[FormatItem<'_>] = &[
    FormatItem::Component(Component::Year(Year::default())),
    FormatItem::Literal(b"-"),
    FormatItem::Component(Component::Month(Month::default())),
    FormatItem::Literal(b"-"),
    FormatItem::Component(Component::Day(Day::default())),
];

const TIME_FORMAT: &[FormatItem<'_>] = &[
    FormatItem::Component(Component::Hour(Hour::default())),
    FormatItem::Literal(b":"),
    FormatItem::Component(Component::Minute(Minute::default())),
    FormatItem::Literal(b":"),
    FormatItem::Component(Component::Second(Second::default())),
];

pub(crate) const NAIVE_DATETIME_FORMAT: &[FormatItem<'_>] = &[
    FormatItem::Compound(DATE_FORMAT),
    FormatItem::Literal(b" "),
    FormatItem::Compound(TIME_FORMAT),
];

pub(crate) mod serde_maybe_date {
    use serde::{Deserialize, Deserializer};

    #[cfg(feature = "serialize")]
    use serde::Serializer;
    use time::{OffsetDateTime, PrimitiveDateTime};

    use super::NAIVE_DATETIME_FORMAT;

    #[cfg(feature = "serialize")]
    pub fn serialize<S: Serializer>(
        date: &Option<OffsetDateTime>,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        match date.map(|date| date.format(NAIVE_DATETIME_FORMAT)) {
            Some(Ok(date)) => s.serialize_some(&date),
            None | Some(Err(_)) => s.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        d: D,
    ) -> Result<Option<OffsetDateTime>, D::Error> {
        let v: &str = match Deserialize::deserialize(d) {
            Ok(v) => v,
            Err(_) => return Ok(None),
        };

        PrimitiveDateTime::parse(v, NAIVE_DATETIME_FORMAT)
            .map(PrimitiveDateTime::assume_utc)
            .map(Some)
            .map_err(serde::de::Error::custom)
    }
}

pub(crate) mod serde_date {
    use serde::{Deserialize, Deserializer};

    #[cfg(feature = "serialize")]
    use serde::Serializer;
    use time::{OffsetDateTime, PrimitiveDateTime};

    use super::NAIVE_DATETIME_FORMAT;

    #[cfg(feature = "serialize")]
    pub fn serialize<S: Serializer>(date: &OffsetDateTime, s: S) -> Result<S::Ok, S::Error> {
        let v = date
            .format(NAIVE_DATETIME_FORMAT)
            .map_err(serde::ser::Error::custom)?;

        s.serialize_str(&v)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<OffsetDateTime, D::Error> {
        let v = String::deserialize(d)?;

        PrimitiveDateTime::parse(&v, NAIVE_DATETIME_FORMAT)
            .map(PrimitiveDateTime::assume_utc)
            .map_err(serde::de::Error::custom)
    }
}
