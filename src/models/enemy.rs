use crate::prelude::Either;

use super::Metadata;

use url::Url;

use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Enemy {
  #[serde(flatten)]
  pub metadata: Metadata,
  pub game_content_links: Either<[!; 0], serde_json::Value>,
  pub game_patch: Option<GamePatch>,
  #[serde(with = "crate::routes::int_bool")]
  pub starts_with_vowel: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GamePatch {
  #[serde(rename = "ID")]
  pub id: u64,
  pub ex_version: u64,
  #[serde(with = "crate::routes::int_bool")]
  pub is_expansion: bool,
  #[serde(flatten)]
  pub names: PatchNames,
  pub release_date: i64,
  pub version: f64,
  #[serde(with = "url_serde")]
  pub banner: Option<Url>,
}

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
