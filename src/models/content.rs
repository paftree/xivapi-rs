//! Content models.

use chrono::{
  DateTime, Utc,
  serde::ts_seconds::deserialize as from_ts,
};

use url::Url;

pub mod achievement;
pub mod action;
pub mod emote;
pub mod enemy;
pub mod fate;
pub mod instance_content;
pub mod item;
pub mod leve;
pub mod minion;
pub mod mount;
pub mod npc;
pub mod place_name;
pub mod quest;
pub mod recipe;
pub mod status;
pub mod title;
pub mod weather;

pub use self::{
  achievement::Achievement,
  action::Action,
  emote::Emote,
  enemy::Enemy,
  fate::Fate,
  instance_content::InstanceContent,
  item::Item,
  leve::Leve,
  minion::Minion,
  mount::Mount,
  npc::Npc,
  place_name::PlaceName,
  quest::Quest,
  recipe::Recipe,
  status::Status,
  title::Title,
  weather::Weather,
};

use super::id::GamePatchId;

/// Content metadata.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Metadata<I> {
  #[serde(rename = "ID")]
  pub id: I,
  #[serde(flatten)]
  pub names: Names,
  pub icon: String,
  pub url: String,
}

/// Content names and their translations.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Names {
  pub name: String,
  #[serde(rename = "Name_de")]
  pub name_de: String,
  #[serde(rename = "Name_en")]
  pub name_en: String,
  #[serde(rename = "Name_fr")]
  pub name_fr: String,
  #[serde(rename = "Name_ja")]
  pub name_ja: String,

  #[serde(flatten)]
  pub plurals: Option<Plurals>,
  #[serde(flatten)]
  pub singulars: Option<Singulars>,
}

/// Plural content names.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Plurals {
  pub plural: String,
  #[serde(rename = "Plural_de")]
  pub plural_de: String,
  #[serde(rename = "Plural_en")]
  pub plural_en: String,
  #[serde(rename = "Plural_fr")]
  pub plural_fr: String,
  #[serde(rename = "Plural_ja")]
  pub plural_ja: String,
}

/// Singular content names.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Singulars {
  pub singular: String,
  #[serde(rename = "Singular_de")]
  pub singular_de: String,
  #[serde(rename = "Singular_en")]
  pub singular_en: String,
  #[serde(rename = "Singular_fr")]
  pub singular_fr: String,
  #[serde(rename = "Singular_ja")]
  pub singular_ja: String,
}

/// A description of a patch.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GamePatch {
  #[serde(rename = "ID")]
  pub id: GamePatchId,
  pub ex_version: i64,
  #[serde(with = "crate::util::serde::int_bool")]
  pub is_expansion: bool,
  #[serde(flatten)]
  pub names: PatchNames,
  #[serde(deserialize_with = "from_ts")]
  pub release_date: DateTime<Utc>,
  pub version: f64,
  #[serde(default, with = "url_serde")]
  pub url: Option<Url>,
  #[serde(with = "url_serde")]
  pub banner: Option<Url>,
}

/// A patch name and its translations.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PatchNames {
  pub name: String,
  #[serde(rename = "Name_cn")]
  pub name_cn: String,
  #[serde(rename = "Name_de")]
  pub name_de: String,
  #[serde(rename = "Name_en")]
  pub name_en: String,
  #[serde(rename = "Name_fr")]
  pub name_fr: String,
  #[serde(rename = "Name_ja")]
  pub name_ja: String,
  #[serde(rename = "Name_kr")]
  pub name_kr: String,
}

macro_rules! route_segment {
  ($($model:ident => $e:expr),+$(,)?) => {
    $(
      impl HasRouteSegment for $model {
        fn route_segment() -> &'static str {
          $e
        }
      }
    )+
  };
  ($($model:ident),+$(,)?) => {
    $(
      impl HasRouteSegment for $model {
        fn route_segment() -> &'static str {
          stringify!($model)
        }
      }
    )+
  };
}

pub trait HasRouteSegment {
  fn route_segment() -> &'static str;
}

route_segment!(
  Achievement,
  Action,
  Emote,
  Fate,
  InstanceContent,
  Item,
  Leve,
  Mount,
  PlaceName,
  Quest,
  Recipe,
  Status,
  Title,
  Weather,
);

route_segment!(
  Enemy => "BNpcName",
  Minion => "Companion",
  Npc => "ENpcResident",
);
