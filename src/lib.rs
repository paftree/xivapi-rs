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

/// The main driver for accessing XIVAPI.
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
  /// Create an API driver without an API key.
  pub fn new() -> Self {
    let client = Client::new();
    XivApi { client, key: None }
  }

  /// Create an API driver with an API key.
  ///
  /// The key will be added to every request made with this instance.
  pub fn with_key<S: AsRef<str>>(key: S) -> Self {
    let client = Client::new();
    let key = Some(key.as_ref().to_string());
    XivApi { client, key }
  }

  crate fn url(&self, path: &str) -> Url {
    let mut base = Url::from_str("https://xivapi.com").unwrap().join(path).unwrap();
    if let Some(ref k) = self.key {
      base.query_pairs_mut().append_pair("key", &k);
    }
    base
  }

  /// Search for a character.
  pub fn character_search(&self) -> CharacterSearchBuilder {
    CharacterSearchBuilder::new(self)
  }

  /// Fetch a specific character by their Lodestone ID.
  pub fn character(&self, id: u64) -> CharacterBuilder {
    CharacterBuilder::new(self, id)
  }

  /// Search for game content.
  pub fn search(&self) -> SearchBuilder {
    SearchBuilder::new(self)
  }

  /// Fetch a specific action by its ID.
  pub fn action(&self, id: u64) -> ContentBuilder<models::Action> {
    ContentBuilder::new(self, id)
  }

  /// Fetch a specific item by its ID.
  pub fn item(&self, id: u64) -> ContentBuilder<models::Item> {
    ContentBuilder::new(self, id)
  }

  /// Fetch a specific achievement by its ID.
  pub fn achievement(&self, id: u64) -> ContentBuilder<models::Achievement> {
    ContentBuilder::new(self, id)
  }

  /// Fetch a specific emote by its ID.
  pub fn emote(&self, id: u64) -> ContentBuilder<models::Emote> {
    ContentBuilder::new(self, id)
  }

  /// Fetch a specific enemy by its ID.
  pub fn enemy(&self, id: u64) -> ContentBuilder<models::Enemy> {
    ContentBuilder::new(self, id)
  }

  /// Fetch a specific fate by its ID.
  pub fn fate(&self, id: u64) -> ContentBuilder<models::Fate> {
    ContentBuilder::new(self, id)
  }

  /// Fetch specific instance content by its ID.
  pub fn instance_content(&self, id: u64) -> ContentBuilder<models::InstanceContent> {
    ContentBuilder::new(self, id)
  }

  /// Fetch a specific leve by its ID.
  pub fn leve(&self, id: u64) -> ContentBuilder<models::Leve> {
    ContentBuilder::new(self, id)
  }

  /// Fetch a specific minion by its ID.
  pub fn minion(&self, id: u64) -> ContentBuilder<models::Minion> {
    ContentBuilder::new(self, id)
  }

  /// Fetch a specific mount by its ID.
  pub fn mount(&self, id: u64) -> ContentBuilder<models::Mount> {
    ContentBuilder::new(self, id)
  }

  /// Fetch a specific NPC by its ID.
  pub fn npc(&self, id: u64) -> ContentBuilder<models::Npc> {
    ContentBuilder::new(self, id)
  }

  /// Fetch a specific place name by its ID.
  pub fn place_name(&self, id: u64) -> ContentBuilder<models::PlaceName> {
    ContentBuilder::new(self, id)
  }

  /// Fetch a specific quest by its ID.
  pub fn quest(&self, id: u64) -> ContentBuilder<models::Quest> {
    ContentBuilder::new(self, id)
  }

  /// Fetch a specific recipe by its ID.
  pub fn recipe(&self, id: u64) -> ContentBuilder<models::Recipe> {
    ContentBuilder::new(self, id)
  }

  /// Fetch a specific status by its ID.
  pub fn status(&self, id: u64) -> ContentBuilder<models::Status> {
    ContentBuilder::new(self, id)
  }

  /// Fetch a specific title by its ID.
  pub fn title(&self, id: u64) -> ContentBuilder<models::Title> {
    ContentBuilder::new(self, id)
  }

  /// Fetch a specific weather by its ID.
  pub fn weather(&self, id: u64) -> ContentBuilder<models::Weather> {
    ContentBuilder::new(self, id)
  }
}
