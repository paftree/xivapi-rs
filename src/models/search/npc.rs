use super::Metadata;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Npc {
  #[serde(flatten)]
  pub metadata: Metadata,
  pub title: String,
}
