use crate::{
  XivApi,
  builder::Builder,
  models::HasRouteSegment,
};

use std::borrow::Cow;

#[derive(Debug, Serialize)]
pub struct ContentBuilder<'x, 'a, O> {
  #[serde(skip)]
  api: &'x XivApi<'x>,

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

  fn api(&self) -> &'x XivApi<'x> {
    self.api
  }

  fn route(&self) -> Cow<str> {
    Cow::Owned(format!("/{}/{}", O::route_segment(), self.id))
  }
}

impl<O> ContentBuilder<'x, 'a, O>
  where for<'de> O: serde::Deserialize<'de>,
{
  pub fn new(api: &'x XivApi<'x>, id: usize) -> Self {
    ContentBuilder {
      api,
      id,
      minified: None,
      columns: None,
      _phantom: Default::default(),
    }
  }

  pub fn minified(&mut self, m: bool) -> &mut Self {
    self.minified = Some(m);
    self
  }

  pub fn columns(&mut self, c: &'a [&'a str]) -> &mut Self {
    self.columns = Some(c);
    self
  }
}

