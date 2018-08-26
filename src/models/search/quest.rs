use super::Metadata;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Quest {
  #[serde(flatten)]
  pub metadata: Metadata,
  pub class_job_level_0: usize,
  pub class_job_level_1: usize,
  #[serde(rename = "JournalGenre.Name")]
  pub journal_genre_name: Option<String>,
  #[serde(rename = "JournalGenre.JournalCategory.Name")]
  pub journal_genre_journal_category_name: Option<String>,
}
