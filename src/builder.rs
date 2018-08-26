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
    route.set_query(Some(&query));

    debug!("route: {:#?}", route);

    let mut res = self.api().client.get(route).send().map_err(Error::Reqwest)?;

    let json = res.json().map_err(Error::Reqwest)?;

    Ok(json)
  }
}
