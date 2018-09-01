use serde::{Deserializer, de};

pub fn deserialize<'de, D>(deserialiser: D) -> Result<u64, D::Error>
  where D: Deserializer<'de>,
{
  struct Visitor;

  impl de::Visitor<'de> for Visitor {
    type Value = u64;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(formatter, "u64 prefixed by `i`")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
      where E: de::Error,
    {
      if s.is_empty() {
        return Err(de::Error::custom(format!("expected string but string was empty")));
      }
      if &s[..1] != "i" {
        return Err(de::Error::custom(format!("expected string to start with `i`")));
      }
      s[1..]
        .parse()
        .map_err(|e| de::Error::custom(format!("could not parse u64: {}", e)))
    }
  }

  deserialiser.deserialize_str(Visitor)
}
