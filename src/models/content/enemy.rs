use crate::prelude::Either;

use super::{Metadata, GamePatch, super::id::EnemyId};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Enemy {
  #[serde(flatten)]
  pub metadata: Metadata<EnemyId>,
  pub game_content_links: Either<[!; 0], serde_json::Value>,
  pub game_patch: Option<GamePatch>,
  #[serde(with = "crate::util::serde::int_bool")]
  pub starts_with_vowel: bool,
}
