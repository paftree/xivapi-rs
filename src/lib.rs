#![feature(in_band_lifetimes, never_type)]

#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;

use reqwest::Client;

use url::Url;

use std::str::FromStr;

mod comma;
pub mod routes;
pub mod builder;
pub mod error;
pub mod models;
pub mod prelude;

use crate::routes::{
  character::{
    CharacterBuilder,
    search::SearchBuilder as CharacterSearchBuilder,
  },
  search::SearchBuilder,
  content::ContentBuilder,
};

#[derive(Debug)]
pub struct XivApi {
  client: Client,
  key: Option<String>,
}

impl Default for XivApi {
  fn default() -> Self {
    XivApi::new()
  }
}

impl XivApi {
  pub fn new() -> Self {
    let client = Client::new();
    XivApi { client, key: None }
  }

  pub fn with_key<S: Into<String>>(key: S) -> Self {
    let client = Client::new();
    let key = Some(key.into());
    XivApi { client, key }
  }

  pub fn url(&self, path: &str) -> Url {
    let mut base = Url::from_str("https://xivapi.com").unwrap().join(path).unwrap();
    if let Some(ref k) = self.key {
      base.query_pairs_mut().append_pair("key", &k);
    }
    base
  }

  pub fn character_search(&self) -> CharacterSearchBuilder {
    CharacterSearchBuilder::new(self)
  }

  pub fn character(&self, id: usize) -> CharacterBuilder {
    CharacterBuilder::new(self, id)
  }

  pub fn search(&self) -> SearchBuilder {
    SearchBuilder::new(self)
  }

  pub fn action(&self, id: usize) -> ContentBuilder<models::Action> {
    ContentBuilder::new(self, id)
  }

  pub fn item(&self, id: usize) -> ContentBuilder<models::Item> {
    ContentBuilder::new(self, id)
  }
}
