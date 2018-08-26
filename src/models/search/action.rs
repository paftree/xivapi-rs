use super::Metadata;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Action {
  #[serde(flatten)]
  pub metadata: Metadata,
  #[serde(rename = "ActionCategory.Name")]
  pub action_category_name: Option<String>,
  #[serde(rename = "ClassJobCategory.Name")]
  pub class_job_category_name: Option<String>,
  pub class_job_level: u64,
}
