use crate::{XivApi, error::*};

use log::debug;

use serde::{Deserialize, Serialize};

use std::borrow::Cow;

pub trait Builder<'x>
  where Self: Serialize,
        for<'de> Self::Output: Deserialize<'de>,
{
  type Output;

  fn api(&self) -> &'x XivApi<'x>;

  fn route(&self) -> Cow<str>;

  fn send(&mut self) -> Result<Self::Output> {
    let mut route = self.api().url(&self.route());

    let query = serde_urlencoded::to_string(&self).map_err(Error::UrlEncode)?;
    let mut route_query = route.query().map(ToString::to_string).unwrap_or_default();
    if !query.is_empty() {
      if !route_query.is_empty() {
        route_query += "&";
      }
      route_query += &query;
    }
    if !route_query.is_empty() {
      route.set_query(Some(&route_query));
    }

    println!("route: {:#?}", route);

    let mut res = self.api().client.get(route).send().map_err(Error::Reqwest)?;

    let text = res.text().map_err(Error::Reqwest)?;

    match serde_json::from_str::<ApiError>(&text) {
      Ok(e) => Err(Error::Api(e)),
      Err(_) => Ok(serde_json::from_str(&text).map_err(Error::Json)?),
    }
  }
}
