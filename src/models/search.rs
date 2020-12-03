//! Search models.

pub mod achievement;
pub mod action;
pub mod character;
pub mod emote;
pub mod enemy;
pub mod fate;
pub mod free_company;
pub mod instance_content;
pub mod item;
pub mod leve;
pub mod linkshell;
pub mod minion;
pub mod mount;
pub mod npc;
pub mod place_name;
pub mod quest;
pub mod recipe;
pub mod status;
pub mod title;
pub mod weather;

#[derive(Debug, Deserialize)]
#[serde(tag = "UrlType")]
pub enum SearchModel {
  Achievement(self::achievement::Achievement),
  Action(self::action::Action),
  Emote(self::emote::Emote),
  #[serde(rename = "BNpcName")]
  Enemy(self::enemy::Enemy),
  Fate(self::fate::Fate),
  InstanceContent(self::instance_content::InstanceContent),
  Item(self::item::Item),
  Leve(self::leve::Leve),
  #[serde(rename = "Companion")]
  Minion(self::minion::Minion),
  Mount(self::mount::Mount),
  #[serde(rename = "ENpcResident")]
  Npc(self::npc::Npc),
  PlaceName(self::place_name::PlaceName),
  Quest(self::quest::Quest),
  Recipe(self::recipe::Recipe),
  Status(self::status::Status),
  Title(self::title::Title),
  Weather(self::weather::Weather),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Clone)]
pub struct Metadata {
  #[serde(rename = "ID")]
  pub id: u64,
  pub name: String,
  pub icon: String,
  pub url: String,
  #[serde(rename = "_")]
  pub index: String,
  #[serde(rename = "_Score")]
  pub score: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchResult {
  pub speed_ms: u64,
  pub pagination: Pagination,
  pub results: Vec<SearchModel>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Pagination {
  pub page: u64,
  pub page_next: Option<u64>,
  pub page_prev: Option<u64>,
  pub page_total: u64,
  pub results: u64,
  pub results_per_page: u64,
  pub results_total: u64,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Index {
  Achievement,
  Action,
  Emote,
  #[serde(rename = "bnpcname")]
  Enemy,
  Fate,
  InstanceContent,
  Item,
  Leve,
  #[serde(rename = "companion")]
  Minion,
  Mount,
  #[serde(rename = "enpcresident")]
  Npc,
  PlaceName,
  Quest,
  Recipe,
  Status,
  Title,
  Weather,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StringAlgo {
  Fuzzy,
  MatchPhrasePrefix,
  MultiMatch,
  Prefix,
  QueryString,
  Term,
  Wildcard,
  WildcardPlus,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum SortOrder {
  #[serde(rename = "asc")]
  Ascending,
  #[serde(rename = "desc")]
  Descending,
}
