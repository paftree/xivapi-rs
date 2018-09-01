use crate::{
  XivApi,
  builder::Builder,
  models::content::HasRouteSegment,
};

use std::borrow::Cow;

/// A generic request builder for content routes.
#[derive(Debug, Serialize)]
pub struct ContentBuilder<'x, 'a, O> {
  #[serde(skip)]
  api: &'x XivApi,

  #[serde(skip)]
  id: u64,

  #[serde(
    skip_serializing_if = "Option::is_none",
    with = "crate::util::serde::int_bool",
  )]
  minified: Option<bool>,

  #[serde(
    skip_serializing_if = "Option::is_none",
    serialize_with = "crate::util::serde::CommaSerializer::with",
  )]
  columns: Option<&'a [&'a str]>,

  #[serde(
    skip_serializing_if = "Option::is_none",
    serialize_with = "crate::util::serde::CommaSerializer::with",
  )]
  tags: Option<&'a [&'a str]>,

  #[serde(skip)]
  _phantom: std::marker::PhantomData<O>,
}

impl<O> Builder<'x> for ContentBuilder<'x, 'a, O>
  where O: HasRouteSegment,
        for<'de> O: serde::Deserialize<'de>,
{
  type Output = O;

  fn api(&self) -> &'x XivApi {
    self.api
  }

  fn route(&self) -> Cow<str> {
    Cow::Owned(format!("/{}/{}", O::route_segment(), self.id))
  }
}

impl<O> ContentBuilder<'x, 'a, O>
  where for<'de> O: serde::Deserialize<'de>,
{
  crate fn new(api: &'x XivApi, id: u64) -> Self {
    ContentBuilder {
      api,
      id,
      minified: None,
      columns: None,
      tags: None,
      _phantom: Default::default(),
    }
  }

  /// Whether to fetch minified results.
  pub fn minified(&mut self, m: bool) -> &mut Self {
    self.minified = Some(m);
    self
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

