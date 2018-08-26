use super::Metadata;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Achievement {
  #[serde(flatten)]
  pub metadata: Metadata,
  pub points: u64,
  #[serde(rename = "AchievementCategory.Name")]
  pub achievement_category_name: String,
  #[serde(rename = "AchievementCategory.AchievementKind.Name")]
  pub achievement_category_achievement_kind_name: String,
}
