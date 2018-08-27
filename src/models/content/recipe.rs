use super::{Metadata, super::id::RecipeId};

use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Recipe {
  #[serde(flatten)]
  pub metadata: Metadata<RecipeId>,
  #[serde(flatten)]
  pub other: BTreeMap<String, serde_json::Value>,
}
