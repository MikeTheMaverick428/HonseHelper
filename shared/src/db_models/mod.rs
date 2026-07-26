pub mod friend_data;
pub mod veteran_data;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct UmaHash(i64);

impl Serialize for UmaHash {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for UmaHash {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de;
        struct UmaHashVisitor;
        impl<'de> de::Visitor<'de> for UmaHashVisitor {
            type Value = UmaHash;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a string representation of an integer")
            }
            fn visit_i64<E: de::Error>(self, v: i64) -> Result<UmaHash, E> {
                Ok(UmaHash(v))
            }
            fn visit_u64<E: de::Error>(self, v: u64) -> Result<UmaHash, E> {
                Ok(UmaHash(v as i64))
            }
            fn visit_str<E: de::Error>(self, v: &str) -> Result<UmaHash, E> {
                v.parse::<i64>().map(UmaHash).map_err(de::Error::custom)
            }
        }
        deserializer.deserialize_any(UmaHashVisitor)
    }
}

impl UmaHash {
    pub fn as_u64(&self) -> u64 {
        self.0 as u64
    }

    pub fn as_i64(&self) -> i64 {
        self.0
    }
}

impl From<i64> for UmaHash {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<u64> for UmaHash {
    fn from(value: u64) -> Self {
        Self(value as i64)
    }
}
