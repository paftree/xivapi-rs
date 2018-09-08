//! All IDs.

macro_rules! id {
  ($($(#[$meta:meta])* $name:ident$(($(#[$inner_meta:meta])*))?);+$(;)?) => {
    $(
      $(#[$meta])*
      #[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Copy)]
      pub struct $name($($(#[$inner_meta])*)? pub u64);

      impl From<u64> for $name {
        fn from(u: u64) -> Self {
          $name(u)
        }
      }

      impl std::ops::Deref for $name {
        type Target = u64;

        fn deref(&self) -> &Self::Target {
          &self.0
        }
      }

      impl $name {
        pub fn into_inner(self) -> u64 {
          self.0
        }
      }
    )+
  }
}

id!(
  AchievementId;
  ActionId;
  CharacterId;
  EmoteId;
  EnemyId;
  FateId;
  FreeCompanyId(#[serde(deserialize_with = "crate::util::serde::u64_str")]);
  GamePatchId;
  InstanceContentId;
  ItemId;
  LeveId;
  LinkshellId(#[serde(deserialize_with = "crate::util::serde::u64_str")]);
  MinionId;
  MountId;
  NpcId;
  PlaceNameId;
  QuestId;
  RecipeId;
  StatusId;
  TitleId;
  WeatherId;
);
