use super::{Metadata, super::id::FateId};

use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Fate {
  #[serde(flatten)]
  pub metadata: Metadata<FateId>,
  #[serde(flatten)]
  pub other: BTreeMap<String, serde_json::Value>,
}
