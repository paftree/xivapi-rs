use super::{Metadata, super::id::MountId};

use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mount {
  #[serde(flatten)]
  pub metadata: Metadata<MountId>,
  #[serde(flatten)]
  pub other: BTreeMap<String, serde_json::Value>,
}
