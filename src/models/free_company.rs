use super::{
  LodestoneInfo,
  id::FreeCompanyId,
};

use chrono::{DateTime, Utc};

use ffxiv_types::World;

use url::Url;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FreeCompanyResult {
  pub info: Info,
  pub free_company: Option<FreeCompany>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Info {
  pub free_company: LodestoneInfo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FreeCompany {
  #[serde(rename = "ID")]
  pub id: FreeCompanyId,
  pub name: String,
  pub tag: String,
  pub server: World,
  pub slogan: String,
  pub active: String,
  pub active_member_count: u64,
  #[serde(deserialize_with = "multi_url")]
  pub crest: Vec<Url>,
  pub estate: Option<Estate>,
  pub focus: Vec<Focus>,
  #[serde(deserialize_with = "crate::util::serde::ts_str")]
  pub formed: DateTime<Utc>,
  pub grand_company: String,
  #[serde(deserialize_with = "crate::util::serde::ts_str")]
  pub parse_date: DateTime<Utc>,
  pub rank: u64,
  pub ranking: Ranking,
  pub recruitment: String,
  pub reputation: Vec<Reputation>,
  pub seeking: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Estate {
  pub greeting: String,
  pub name: String,
  pub plot: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Focus {
  pub status: bool,
  #[serde(with = "url_serde")]
  pub icon: Url,
  pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ranking {
  #[serde(deserialize_with = "ranking_u64")]
  pub monthly: Option<u64>,
  #[serde(deserialize_with = "ranking_u64")]
  pub weekly: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Reputation {
  pub name: String,
  pub rank: String,
  pub progress: u64,
}

fn ranking_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
  where D: serde::Deserializer<'de>,
{
  struct Visitor;

  impl serde::de::Visitor<'de> for Visitor {
    type Value = Option<u64>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(formatter, "u64 or `--`")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
      where E: serde::de::Error,
    {
      if s == "--" {
        Ok(None)
      } else {
        Err(serde::de::Error::invalid_value(
          serde::de::Unexpected::Str(s),
          &self,
        ))
      }
    }

    fn visit_u64<E>(self, u: u64) -> Result<Self::Value, E>
      where E: serde::de::Error,
    {
      Ok(Some(u))
    }
  }

  deserializer.deserialize_any(Visitor)
}

crate fn multi_url<'de, D>(deserializer: D) -> Result<Vec<Url>, D::Error>
  where D: serde::Deserializer<'de>,
{
  use serde::Deserialize;

  #[derive(Deserialize)]
  struct Wrapper(#[serde(with = "url_serde")] Url);

  let v = Vec::deserialize(deserializer)?;

  Ok(v.into_iter().map(|Wrapper(u)| u).collect())
}
