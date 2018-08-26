use super::Metadata;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Emote {
  #[serde(flatten)]
  pub metadata: Metadata,
  #[serde(rename = "EmoteCategory.Name")]
  pub emote_category_name: String,
  #[serde(rename = "TextCommand.Command")]
  pub text_command_command: Option<String>,
}
