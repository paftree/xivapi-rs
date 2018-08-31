use serde::{
  Serializer, Deserializer,
  de,
};

pub fn serialize<'a, S>(input: &'a Option<bool>, serialiser: S) -> Result<S::Ok, S::Error>
  where S: Serializer,
{
  let input = match input {
    Some(a) => a,
    None => return serialiser.serialize_none(),
  };
  if *input {
    serialiser.serialize_u8(1)
  } else {
    serialiser.serialize_u8(0)
  }
}

pub fn deserialize<'de, D>(deserialiser: D) -> Result<bool, D::Error>
  where D: Deserializer<'de>,
{
  struct Visitor;

  impl de::Visitor<'de> for Visitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(formatter, "0 or 1 or a boolean")
    }

    fn visit_bool<E>(self, b: bool) -> Result<Self::Value, E>
      where E: de::Error,
    {
      Ok(b)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
      where E: de::Error,
    {
      match v {
        0 => Ok(false),
        1 => Ok(true),
        x => Err(de::Error::custom(format!("expected 0 or 1 but found {}", x))),
      }
    }
  }

  deserialiser.deserialize_any(Visitor)
}
