use std::{
    fmt::{Formatter, Result as FmtResult},
    marker::PhantomData,
};

use serde::de::{Deserialize, IgnoredAny, SeqAccess, Visitor};

#[derive(Default)]
pub(crate) struct SingleItemVisitor<T> {
    phantom: PhantomData<T>,
}

impl<'de, T: Deserialize<'de>> Visitor<'de> for SingleItemVisitor<T> {
    type Value = Option<T>;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a sequence")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let item = seq.next_element()?;
        while seq.next_element::<IgnoredAny>()?.is_some() {}

        Ok(item)
    }
}
