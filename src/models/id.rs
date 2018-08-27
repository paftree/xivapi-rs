macro_rules! id {
  ($($(#[$meta:meta])* $name:ident);+$(;)?) => {
    $(
      $(#[$meta])*
      #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
      pub struct $name(pub u64);

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

      impl serde::Serialize for $name {
        fn serialize<S>(&self, ser: S) -> std::result::Result<S::Ok, S::Error>
          where S: serde::Serializer,
        {
          use serde::Serialize;
          self.0.serialize(ser)
        }
      }

      impl serde::Deserialize<'de> for $name {
        fn deserialize<D>(des: D) -> std::result::Result<Self, D::Error>
          where D: serde::Deserializer<'de>
        {
          use serde::Deserialize;
          u64::deserialize(des).map($name)
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
  GamePatchId;
  InstanceContentId;
  ItemId;
  LeveId;
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
