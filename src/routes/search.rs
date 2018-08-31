use crate::{
  XivApi,
  builder::Builder,
  models::search::{
    Index,
    SearchResult,
    SortOrder,
    StringAlgo,
  },
};

use std::borrow::Cow;

/// A request builder for searching for content on XIVAPI.
#[derive(Debug, Serialize)]
pub struct SearchBuilder<'x, 'a> {
  #[serde(skip)]
  api: &'x XivApi,

  #[serde(skip_serializing_if = "Option::is_none")]
  string: Option<&'a str>,

  #[serde(
    skip_serializing_if = "Option::is_none",
    serialize_with = "crate::util::serde::comma::CommaSerializer::with",
  )]
  indexes: Option<Vec<Index>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  string_column: Option<&'a str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  string_algo: Option<StringAlgo>,

  #[serde(skip_serializing_if = "Option::is_none")]
  page: Option<usize>,

  #[serde(skip_serializing_if = "Option::is_none")]
  sort_field: Option<&'a str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  sort_order: Option<SortOrder>,

  #[serde(skip_serializing_if = "Option::is_none")]
  limit: Option<usize>,

  #[serde(
    skip_serializing_if = "Option::is_none",
    serialize_with = "crate::util::serde::comma::CommaSerializer::with",
  )]
  filters: Option<&'a [&'a str]>,

  #[serde(
    skip_serializing_if = "Option::is_none",
    serialize_with = "crate::util::serde::comma::CommaSerializer::with",
  )]
  tags: Option<&'a [&'a str]>,
}

impl Builder<'x> for SearchBuilder<'x, 'a> {
  type Output = SearchResult;

  fn api(&self) -> &'x XivApi {
    self.api
  }

  fn route(&self) -> Cow<str> {
    Cow::Borrowed("/Search")
  }
}

impl SearchBuilder<'x, 'a> {
  crate fn new(api: &'x XivApi) -> Self {
    SearchBuilder {
      api,
      string: None,
      indexes: None,
      string_column: None,
      string_algo: None,
      page: None,
      sort_field: None,
      sort_order: None,
      limit: None,
      filters: None,
      tags: None,
    }
  }

  /// Specify what to search for.
  pub fn string(&mut self, s: &'a str) -> &mut Self {
    self.string = Some(s);
    self
  }

  /// Select which type of content to search for.
  ///
  /// This is additive. Each call to this function will add another type of content to search for.
  pub fn index(&mut self, i: Index) -> &mut Self {
    self.indexes.get_or_insert_with(Default::default).push(i);
    self
  }

  /// Select which type of content to search for.
  ///
  /// This is additive. Each call to this function will add on to the already specified types, if
  /// any.
  pub fn indexes(&mut self, mut is: Vec<Index>) -> &mut Self {
    self.indexes.get_or_insert_with(Default::default).append(&mut is);
    self
  }

  pub fn string_column(&mut self, s: &'a str) -> &mut Self {
    self.string_column = Some(s);
    self
  }

  pub fn string_algo(&mut self, a: StringAlgo) -> &mut Self {
    self.string_algo = Some(a);
    self
  }

  /// Pick which page to fetch.
  pub fn page(&mut self, p: usize) -> &mut Self {
    self.page = Some(p);
    self
  }

  pub fn sort_field(&mut self, f: &'a str) -> &mut Self {
    self.sort_field = Some(f);
    self
  }

  pub fn sort_order(&mut self, o: SortOrder) -> &mut Self {
    self.sort_order = Some(o);
    self
  }

  /// Change the limit for how many results are fetched.
  pub fn limit(&mut self, l: usize) -> &mut Self {
    self.limit = Some(l);
    self
  }

  /// Set filters on what is fetched.
  pub fn filters(&mut self, f: &'a [&'a str]) -> &mut Self {
    self.filters = Some(f);
    self
  }

  /// Set tracking tags on this request.
  pub fn tags(&mut self, tags: &'a [&'a str]) -> &mut Self {
    self.tags = Some(tags);
    self
  }
}

