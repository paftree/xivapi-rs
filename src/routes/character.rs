use crate::{
  XivApi,
  builder::Builder,
  models::{character::CharacterResult, id::CharacterId},
};

use std::borrow::Cow;

pub mod search;

/// A builder for fetching character information from XIVAPI.
#[derive(Debug, Serialize)]
pub struct CharacterBuilder<'x, 'a> {
  #[serde(skip)]
  api: &'x XivApi,

  #[serde(skip)]
  id: CharacterId,

  #[serde(
    skip_serializing_if = "Option::is_none",
    serialize_with = "crate::util::serde::comma::CommaSerializer::with",
  )]
  columns: Option<&'a [&'a str]>,

  #[serde(
    skip_serializing_if = "Option::is_none",
    serialize_with = "crate::util::serde::comma::CommaSerializer::with",
  )]
  tags: Option<&'a [&'a str]>,
}

impl Builder<'x> for CharacterBuilder<'x, 'a> {
  type Output = CharacterResult;

  fn api(&self) -> &'x XivApi {
    self.api
  }

  fn route(&self) -> Cow<str> {
    Cow::Owned(format!("/Character/{}", self.id.0))
  }
}

impl CharacterBuilder<'x, 'a> {
  crate fn new(api: &'x XivApi, id: CharacterId) -> Self {
    CharacterBuilder {
      api,
      id,
      columns: Some(&["Character", "Info.Character"]),
      tags: None,
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

  /// Set tracking tags on this request.
  pub fn tags(&mut self, tags: &'a [&'a str]) -> &mut Self {
    self.tags = Some(tags);
    self
  }
}

