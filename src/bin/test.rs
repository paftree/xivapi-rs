#[macro_use] extern crate serde_derive;

use xivapi::{
  prelude::*,
  models::search::Index,
  models::character::{Race, Gender, State},
};

fn main() -> Result<(), failure::Error> {
  let api = XivApi::with_key("d0b5477b34c944e3b87b8a46");

  // let res = api
  //   .character_search()
  //   .name("Duvivi Duvi")
  //   .server(World::Adamantoise)
  //   .send()?;

  // let id = res.characters[0].id;

  let res: CharInfoResult = api
    .character(1)
    .columns(&["Name", "Server", "Race", "Gender"])
    .json()?;

  println!("{:#?}", res);

  Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CharInfoResult {
  state: State,
  payload: Option<CharInfo>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CharInfo {
  name: String,
  race: Race,
  gender: Gender,
  server: World,
}
