use super::Metadata;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Minion {
  #[serde(flatten)]
  pub metadata: Metadata,
  #[serde(rename = "MinionRace.Name")]
  pub minion_race_name: String,
  #[serde(rename = "Behavior.Name")]
  pub behavior_name: String,
}
