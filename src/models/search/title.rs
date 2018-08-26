use super::Metadata;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Title {
  #[serde(flatten)]
  pub metadata: Metadata,
  pub name_female: String,
}
