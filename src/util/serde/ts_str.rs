use chrono::{DateTime, TimeZone, Utc};

use serde::Deserialize;

crate fn ts_str<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
  where D: serde::de::Deserializer<'de>,
{
  let t = String::deserialize(deserializer)?;

  let ts: i64 = t
    .parse()
    .map_err(|_| serde::de::Error::invalid_value(
      serde::de::Unexpected::Str(&t),
      &"string containing a signed 64-bit integer",
    ))?;

  Ok(Utc.timestamp(ts, 0))
}
