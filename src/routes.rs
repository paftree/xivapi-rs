crate mod character;
crate mod content;
crate mod search;

#[derive(Debug, Serialize)]
pub enum Language {
  #[serde(rename = "cn")]
  Chinese,
  #[serde(rename = "en")]
  English,
  #[serde(rename = "fr")]
  French,
  #[serde(rename = "de")]
  German,
  #[serde(rename = "ja")]
  Japanese,
  #[serde(rename = "kr")]
  Korean,
}

pub fn bool_as_usize<'a, S>(input: &'a Option<bool>, serialiser: S) -> Result<S::Ok, S::Error>
  where S: serde::Serializer,
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
