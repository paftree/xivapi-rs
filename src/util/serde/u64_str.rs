use serde::Deserialize;

crate fn u64_str<'de, D>(deserializer: D) -> Result<u64, D::Error>
  where D: serde::Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  s
    .parse()
    .map_err(|_| serde::de::Error::invalid_value(
      serde::de::Unexpected::Str(&s),
      &"string containing a u64",
    ))
}
