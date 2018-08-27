use super::{Metadata, super::id::QuestId};

use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Quest {
  #[serde(flatten)]
  pub metadata: Metadata<QuestId>,
  #[serde(flatten)]
  pub other: BTreeMap<String, serde_json::Value>,
}
