use super::Metadata;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mount {
  #[serde(flatten)]
  pub metadata: Metadata,
}
