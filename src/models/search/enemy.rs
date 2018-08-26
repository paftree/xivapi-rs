use super::Metadata;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Enemy {
  #[serde(flatten)]
  pub metadata: Metadata,
}
