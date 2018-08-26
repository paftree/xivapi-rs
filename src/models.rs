pub mod action;
pub mod character;
pub mod item;
pub mod search;

pub use self::{
  action::Action,
  item::Item,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Metadata {
  #[serde(rename = "ID")]
  pub id: usize,
  #[serde(flatten)]
  pub names: Names,
  pub icon: String,
  pub url: String,
}

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
  pub plurals: Plurals,
  #[serde(flatten)]
  pub singulars: Singulars,
}

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

pub trait HasRouteSegment {
  fn route_segment() -> &'static str;
}

impl HasRouteSegment for Action {
  fn route_segment() -> &'static str {
    "Action"
  }
}

impl HasRouteSegment for Item {
  fn route_segment() -> &'static str {
    "Item"
  }
}
