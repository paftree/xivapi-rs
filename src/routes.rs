pub mod character;
pub mod content;
pub mod free_company;
pub mod search;

#[derive(Debug, Serialize)]
pub enum Language {
  #[serde(rename = "cn")]
  Chinese,
  #[serde(rename = "en")]
  English,
  #[serde(rename = "fr")]
  French,
  #[serde(rename = "de")]
  German,
  #[serde(rename = "ja")]
  Japanese,
  #[serde(rename = "kr")]
  Korean,
}
