use super::Metadata;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Fate {
  #[serde(flatten)]
  pub metadata: Metadata,
  pub class_job_level: usize,
  pub class_job_level_max: usize,
}
