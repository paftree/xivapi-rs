use crate::models::id::FreeCompanyId;

use super::LodestonePagination;

use url::Url;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchResult {
  #[serde(flatten)]
  pub pagination: LodestonePagination,
  pub free_companies: Vec<SearchFreeCompany>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchFreeCompany {
  #[serde(rename = "ID")]
  pub id: FreeCompanyId,
  pub name: String,
  #[serde(deserialize_with = "crate::models::free_company::multi_url")]
  pub crest: Vec<Url>,
}
