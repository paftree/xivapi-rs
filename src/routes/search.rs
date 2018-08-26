use crate::{
  XivApi,
  builder::Builder,
  comma::CommaSerializer,
  models::search::{
    Index,
    SearchResult,
    SortOrder,
    StringAlgo,
    StringColumn,
  },
};

use std::borrow::Cow;

#[derive(Debug, Serialize)]
pub struct SearchBuilder<'x, 'a> {
  #[serde(skip)]
  api: &'x XivApi<'x>,

  #[serde(skip_serializing_if = "Option::is_none")]
  string: Option<&'a str>,

  #[serde(
    skip_serializing_if = "Option::is_none",
    serialize_with = "CommaSerializer::with",
  )]
  indexes: Option<Vec<Index>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  string_column: Option<StringColumn>,

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
    serialize_with = "CommaSerializer::with",
  )]
  filters: Option<&'a [&'a str]>,
}

impl Builder<'x> for SearchBuilder<'x, 'a> {
  type Output = SearchResult;

  fn api(&self) -> &'x XivApi<'x> {
    self.api
  }

  fn route(&self) -> Cow<str> {
    Cow::Borrowed("/Search")
  }
}

impl SearchBuilder<'x, 'a> {
  pub fn new(api: &'x XivApi<'x>) -> Self {
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
    }
  }

  pub fn string(&mut self, s: &'a str) -> &mut Self {
    self.string = Some(s);
    self
  }

  pub fn index(&mut self, i: Index) -> &mut Self {
    self.indexes.get_or_insert_with(Default::default).push(i);
    self
  }

  pub fn indexes(&mut self, mut is: Vec<Index>) -> &mut Self {
    self.indexes.get_or_insert_with(Default::default).append(&mut is);
    self
  }

  pub fn string_column(&mut self, s: StringColumn) -> &mut Self {
    self.string_column = Some(s);
    self
  }

  pub fn string_algo(&mut self, a: StringAlgo) -> &mut Self {
    self.string_algo = Some(a);
    self
  }

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

  pub fn limit(&mut self, l: usize) -> &mut Self {
    self.limit = Some(l);
    self
  }

  pub fn filters(&mut self, f: &'a [&'a str]) -> &mut Self {
    self.filters = Some(f);
    self
  }
}

