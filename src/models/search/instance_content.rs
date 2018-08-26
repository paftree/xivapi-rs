use super::Metadata;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InstanceContent {
  #[serde(flatten)]
  pub metadata: Metadata,
  #[serde(rename = "ContentType.Name")]
  pub content_type_name: String,
  #[serde(rename = "TerritoryType.PlaceName.Name")]
  pub territory_type_place_name_name: Option<String>,
  #[serde(flatten)]
  pub content_finder_conditions: ContentFinderConditions,
}

#[derive(Debug, Deserialize)]
pub struct ContentFinderConditions {
  #[serde(rename = "ContentFinderCondition.ClassJobLevelRequired")]
  pub class_job_level_required: u64,
  #[serde(rename = "ContentFinderCondition.ClassJobLevelSync")]
  pub class_job_level_sync: u64,
  #[serde(rename = "ContentFinderCondition.ItemLevelRequired")]
  pub item_level_required: u64,
  #[serde(rename = "ContentFinderCondition.ItemLevelSync")]
  pub item_level_sync: u64,
}
