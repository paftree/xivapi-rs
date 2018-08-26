use crate::{
  XivApi,
  builder::Builder,
  models::character::CharacterResult,
};

use ffxiv_types::World;

use std::borrow::Cow;

pub mod search;

#[derive(Debug, Serialize)]
pub struct CharacterBuilder<'x, 'a> {
  #[serde(skip)]
  api: &'x XivApi<'x>,

  #[serde(skip)]
  id: usize,

  #[serde(skip_serializing_if = "Option::is_none")]
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
  pub fn new(api: &'x XivApi<'x>, id: usize) -> Self {
    CharacterBuilder {
      api,
      id,
      columns: None,
    }
  }

  pub fn columns(&mut self, c: &'a [&'a str]) -> &mut Self {
    self.columns = Some(c);
    self
  }
}

