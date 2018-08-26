pub use crate::{
  XivApi,
  builder::Builder,
};

pub use ffxiv_types::World;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Either<L, R> {
  Left(L),
  Right(R),
}
