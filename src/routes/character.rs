use crate::{
  XivApi,
  builder::Builder,
  comma::CommaSerializer,
  models::character::CharacterResult,
};

use ffxiv_types::World;

use std::borrow::Cow;

pub mod search;

/// A builder for fetching character information from XIVAPI.
#[derive(Debug, Serialize)]
pub struct CharacterBuilder<'x, 'a> {
  #[serde(skip)]
  api: &'x XivApi<'x>,

  #[serde(skip)]
  id: usize,

  #[serde(
    skip_serializing_if = "Option::is_none",
    serialize_with = "CommaSerializer::with",
  )]
  columns: Option<&'a [&'a str]>,
}

impl Builder<'x> for CharacterBuilder<'x, 'a> {
  type Output = CharacterResult;

  fn api(&self) -> &'x XivApi<'x> {
    self.api
  }

  fn route(&self) -> Cow<str> {
    Cow::Owned(format!("/Character/{}", self.id))
  }
}

impl CharacterBuilder<'x, 'a> {
  crate fn new(api: &'x XivApi<'x>, id: usize) -> Self {
    CharacterBuilder {
      api,
      id,
      columns: None,
    }
  }

  /// Pick which columns to fetch.
  ///
  /// # Note
  /// If using this, the builder must not be finished by calling `send`, as the output will not be
  /// the default output. You will need to create your own data structure to deserialise into, then
  /// call `json`.
  pub fn columns(&mut self, c: &'a [&'a str]) -> &mut Self {
    self.columns = Some(c);
    self
  }
}

