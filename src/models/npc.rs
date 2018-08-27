use super::{Metadata, id::NpcId};

use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Npc {
  #[serde(flatten)]
  pub metadata: Metadata<NpcId>,
  #[serde(flatten)]
  pub other: BTreeMap<String, serde_json::Value>,
}
