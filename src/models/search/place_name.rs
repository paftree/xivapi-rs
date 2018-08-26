use super::Metadata;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaceName {
  #[serde(flatten)]
  pub metadata: Metadata,
}
