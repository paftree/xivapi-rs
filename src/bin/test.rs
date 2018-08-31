#![feature(never_type)]

#[macro_use] extern crate serde_derive;

use xivapi::{
  prelude::*,
  models::search::Index,
  models::character::{Race, Gender, State},
};

fn main() -> Result<(), failure::Error> {
  let key = std::env::var("XIVAPI_KEY").unwrap();
  let api = XivApi::with_key(&key);

  // let res = api
  //   .character_search()
  //   .name("Duvivi Duvi")
  //   .server(World::Adamantoise)
  //   .send()?;

  // let id = res.characters[0].id;

  // let res: CharInfoResult = api
  //   .character(1)
  //   .columns(&["Name", "Server", "Race", "Gender"])
  //   .json()?;

  // let res = api
  //   .character(1)
  //   .send()?;

  // let res = api.enemy(7537.into()).send()?;
  let res = api.character(2.into()).send()?;

  println!("{:#?}", res);

  Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CharInfoResult {
  state: State,
  payload: Either<CharInfo, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CharInfo {
  name: String,
  race: Race,
  gender: Gender,
  server: World,
}
