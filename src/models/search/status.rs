use super::Metadata;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Status {
  #[serde(flatten)]
  pub metadata: Metadata,
  pub description: String,
}
