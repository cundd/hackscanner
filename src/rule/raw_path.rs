use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

use serde::{Deserialize, Deserializer};
use serde::de::{self, Visitor, MapAccess};

#[derive(Debug, Clone, Deserialize, PartialOrd, PartialEq)]
pub struct RawPath {
    pattern: String,
    is_regex: bool,
}

impl RawPath {
    pub fn new<S: Into<String>>(pattern: S, is_regex: bool) -> Self {
        Self {
            pattern: pattern.into(),
            is_regex,
        }
    }
    pub fn with_regex<S: Into<String>>(pattern: S) -> Self {
        Self {
            pattern: pattern.into(),
            is_regex: true,
        }
    }
    pub fn with_path<S: Into<String>>(pattern: S) -> Self {
        Self {
            pattern: pattern.into(),
            is_regex: false,
        }
    }

    pub fn as_str(&self) -> &str {
        self.pattern.as_str()
    }

    pub fn is_regex(&self) -> bool {
        self.is_regex
    }
}

// impl std::convert::From<&str> for RawPath {
//     fn from(pattern: &str) -> Self {
//         RawPath {
//             pattern: pattern.to_owned(),
//             is_regex: false,
//         }
//     }
// }

// The `string_or_struct` function uses this impl to instantiate a `RawPath` if
// the input file contains a string and not a struct.
//
// > `path` can be specified either as a string containing the path, or an object with the
// > path and `is_regex` boolean
impl FromStr for RawPath {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RawPath {
            pattern: s.to_string(),
            is_regex: false,
        })
    }
}

pub fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: Deserialize<'de> + FromStr<Err=()>,
        D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
        where
            T: Deserialize<'de> + FromStr<Err=()>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
            where
                E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
            where
                M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}
