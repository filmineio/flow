use serde::{de, Deserialize, Deserializer};
use std::str::FromStr;

pub fn de_u16_from_str<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    u16::from_str(&s).map_err(de::Error::custom)
}
