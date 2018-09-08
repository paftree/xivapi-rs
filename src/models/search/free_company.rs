use crate::models::id::FreeCompanyId;

use super::Pagination;

use url::Url;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchResult {
  pub pagination: Pagination,
  pub results: Vec<SearchFreeCompany>,
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
