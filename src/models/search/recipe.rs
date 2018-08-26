use super::Metadata;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Recipe {
  #[serde(flatten)]
  pub metadata: Metadata,
  #[serde(rename = "ClassJob.Name")]
  pub class_job_name: Option<String>,
  #[serde(rename = "SecretRecipeBook.Name")]
  pub secret_recipe_book_name: Option<String>,
  #[serde(rename = "RecipeLevelTable.Stars")]
  pub recipe_level_table_stars: u64,
  #[serde(rename = "RecipeLevelTable.ClassJobLevel")]
  pub recipe_level_table_class_job_level: u64,
}
