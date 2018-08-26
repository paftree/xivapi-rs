use super::Metadata;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Weather {
  #[serde(flatten)]
  pub metadata: Metadata,
  pub description: String,
}
