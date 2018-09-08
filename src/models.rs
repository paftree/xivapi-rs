//! The models representing API responses.

use chrono::{
  DateTime, Utc,
  serde::ts_seconds::deserialize as from_ts,
};

macro_rules! enum_number {
  ($name:ident { $($variant:ident = $value:expr, )* }) => {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum $name {
      $($variant = $value,)*
    }

    impl serde::Serialize for $name {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer,
      {
        serializer.serialize_u64(*self as u64)
      }
    }

    impl<'de> serde::Deserialize<'de> for $name {
      fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>,
      {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
          type Value = $name;

          fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("positive integer")
          }

          fn visit_u64<E>(self, value: u64) -> Result<$name, E>
            where E: serde::de::Error,
          {
            match value {
              $( $value => Ok($name::$variant), )*
              _ => Err(E::custom(
                  format!("unknown {} value: {}",
                  stringify!($name), value))),
            }
          }
        }

        deserializer.deserialize_u64(Visitor)
      }
    }
  }
}

pub mod character;
pub mod content;
pub mod free_company;
pub mod id;
pub mod linkshell;
pub mod search;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LodestoneInfo {
  pub state: State,
  // #[serde(deserialize_with = "optional_timestamp")]
  // pub updated: Option<DateTime<Utc>>,
  pub updated: Option<String>,
}

enum_number!(State {
  None = 0,
  Adding = 1,
  Cached = 2,
  NotFound = 3,
  Blacklist = 4,
  Private = 5,
});

fn optional_timestamp<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
  where D: serde::de::Deserializer<'de>,
{
  use serde::Deserialize;

  #[derive(Deserialize)]
  struct Wrapper(#[serde(deserialize_with = "from_ts")] DateTime<Utc>);

  match Option::deserialize(deserializer)? {
    Some(Wrapper(t)) => Ok(Some(t)),
    None => Ok(None),
  }
}
