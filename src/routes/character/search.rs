use crate::{
  XivApi,
  builder::Builder,
  models::character::{
    search::SearchResult,
  },
};

use ffxiv_types::World;

use std::borrow::Cow;

/// A builder for searching for a character on XIVAPI.
#[derive(Debug, Serialize)]
pub struct SearchBuilder<'x, 'a> {
  #[serde(skip)]
  api: &'x XivApi,

  #[serde(skip_serializing_if = "Option::is_none")]
  name: Option<&'a str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  server: Option<World>,

  #[serde(skip_serializing_if = "Option::is_none")]
  page: Option<usize>,
}

impl Builder<'x> for SearchBuilder<'x, 'a> {
  type Output = SearchResult;

  fn api(&self) -> &'x XivApi {
    self.api
  }

  fn route(&self) -> Cow<str> {
    Cow::Borrowed("/Character/Search")
  }
}

impl SearchBuilder<'x, 'a> {
  crate fn new(api: &'x XivApi) -> Self {
    SearchBuilder {
      api,
      name: None,
      server: None,
      page: None,
    }
  }

  /// Specify the name to search for.
  pub fn name(&mut self, n: &'a str) -> &mut Self {
    self.name = Some(n);
    self
  }

  /// Specify the server to search on.
  pub fn server(&mut self, s: World) -> &mut Self {
    self.server = Some(s);
    self
  }

  // Select the page of results to view.
  pub fn page(&mut self, p: usize) -> &mut Self {
    self.page = Some(p);
    self
  }
}
