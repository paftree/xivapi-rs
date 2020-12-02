use super::Metadata;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Item {
  #[serde(flatten)]
  pub metadata: Metadata,
  #[serde(default)]
  pub level_equip: Option<u64>,
  #[serde(default)]
  pub level_item: Option<u64>,
  #[serde(rename = "ClassJobCategory.Name")]
  pub class_job_category_name: Option<String>,
  #[serde(rename = "ItemUICategory.Name")]
  pub item_ui_category_name: Option<String>,
  #[serde(rename = "ItemKind.Name")]
  pub item_kind_name: Option<String>,
  #[serde(rename = "ItemSearchCategory.Name")]
  pub item_search_category_name: Option<String>,
  pub rarity: Option<u64>,
}
