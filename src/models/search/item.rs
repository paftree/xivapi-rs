use super::Metadata;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Item {
  #[serde(flatten)]
  pub metadata: Metadata,
  #[serde(default)]
  pub level_equip: Option<usize>,
  #[serde(default)]
  pub level_item: Option<usize>,
  #[serde(rename = "ClassJobCategory.Name")]
  pub class_job_category_name: Option<String>,
  #[serde(rename = "ItemUICategory.Name")]
  pub item_ui_category_name: Option<String>,
  #[serde(rename = "ItemKind.Name")]
  pub item_kind_name: String,
  #[serde(rename = "ItemSearchCategory.Name")]
  pub item_search_category_name: Option<String>,
  pub rarity: usize,
}
