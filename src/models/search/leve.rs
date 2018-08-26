use super::Metadata;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Leve {
  #[serde(flatten)]
  pub metadata: Metadata,
  pub class_job_level: u64,
  #[serde(rename = "ClassJobCategory.Name")]
  pub class_job_category_name: String,
  #[serde(rename = "JournalGenre.Name")]
  pub journal_genre_name: String,
  #[serde(rename = "JournalGenre.JournalCategory.Name")]
  pub journal_genre_journal_category_name: String,
}
