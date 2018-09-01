use super::LodestonePagination;

use ffxiv_types::World;

use url::Url;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchResult {
  #[serde(flatten)]
  pub pagination: LodestonePagination,
  pub characters: Vec<SearchCharacter>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchCharacter {
  #[serde(rename = "ID")]
  pub id: u64,
  pub name: String,
  pub server: World,
  #[serde(with = "url_serde")]
  pub avatar: Url,
  pub rank: Option<serde_json::Value>,
  pub rank_icon: Option<serde_json::Value>,
  pub feast_matches: u64,
}
