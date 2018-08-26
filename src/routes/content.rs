use crate::{
  XivApi,
  builder::Builder,
  models::HasRouteSegment,
};

use std::borrow::Cow;

/// A generic request builder for content routes.
#[derive(Debug, Serialize)]
pub struct ContentBuilder<'x, 'a, O> {
  #[serde(skip)]
  api: &'x XivApi,

  #[serde(skip)]
  id: usize,

  #[serde(
    skip_serializing_if = "Option::is_none",
    serialize_with = "crate::routes::bool_as_usize",
  )]
  minified: Option<bool>,

  #[serde(skip_serializing_if = "Option::is_none")]
  columns: Option<&'a [&'a str]>,

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
  crate fn new(api: &'x XivApi, id: usize) -> Self {
    ContentBuilder {
      api,
      id,
      minified: None,
      columns: None,
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
}

