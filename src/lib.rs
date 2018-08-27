#![feature(in_band_lifetimes, never_type, macro_at_most_once_rep)]

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

  pub fn with_key<S: AsRef<str>>(key: S) -> Self {
    let client = Client::new();
    let key = Some(key.as_ref().to_string());
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

  pub fn character(&self, id: u64) -> CharacterBuilder {
    CharacterBuilder::new(self, id)
  }

  pub fn search(&self) -> SearchBuilder {
    SearchBuilder::new(self)
  }

  pub fn action(&self, id: u64) -> ContentBuilder<models::Action> {
    ContentBuilder::new(self, id)
  }

  pub fn item(&self, id: u64) -> ContentBuilder<models::Item> {
    ContentBuilder::new(self, id)
  }

  pub fn achievement(&self, id: u64) -> ContentBuilder<models::Achievement> {
    ContentBuilder::new(self, id)
  }

  pub fn emote(&self, id: u64) -> ContentBuilder<models::Emote> {
    ContentBuilder::new(self, id)
  }

  pub fn enemy(&self, id: u64) -> ContentBuilder<models::Enemy> {
    ContentBuilder::new(self, id)
  }

  pub fn fate(&self, id: u64) -> ContentBuilder<models::Fate> {
    ContentBuilder::new(self, id)
  }

  pub fn instance_content(&self, id: u64) -> ContentBuilder<models::InstanceContent> {
    ContentBuilder::new(self, id)
  }

  pub fn leve(&self, id: u64) -> ContentBuilder<models::Leve> {
    ContentBuilder::new(self, id)
  }

  pub fn minion(&self, id: u64) -> ContentBuilder<models::Minion> {
    ContentBuilder::new(self, id)
  }

  pub fn mount(&self, id: u64) -> ContentBuilder<models::Mount> {
    ContentBuilder::new(self, id)
  }

  pub fn npc(&self, id: u64) -> ContentBuilder<models::Npc> {
    ContentBuilder::new(self, id)
  }

  pub fn place_name(&self, id: u64) -> ContentBuilder<models::PlaceName> {
    ContentBuilder::new(self, id)
  }

  pub fn quest(&self, id: u64) -> ContentBuilder<models::Quest> {
    ContentBuilder::new(self, id)
  }

  pub fn recipe(&self, id: u64) -> ContentBuilder<models::Recipe> {
    ContentBuilder::new(self, id)
  }

  pub fn status(&self, id: u64) -> ContentBuilder<models::Status> {
    ContentBuilder::new(self, id)
  }

  pub fn title(&self, id: u64) -> ContentBuilder<models::Title> {
    ContentBuilder::new(self, id)
  }

  pub fn weather(&self, id: u64) -> ContentBuilder<models::Weather> {
    ContentBuilder::new(self, id)
  }
}
