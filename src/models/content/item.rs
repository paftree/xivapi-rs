use super::{Metadata, super::id::ItemId};

use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Clone)]
pub struct Item {
  #[serde(flatten)]
  #[derive(Clone)]
  pub metadata: Metadata<ItemId>,
  #[serde(flatten)]
  pub other: BTreeMap<String, serde_json::Value>,
}
