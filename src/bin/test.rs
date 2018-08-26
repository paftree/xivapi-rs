use xivapi::{
  prelude::*,
  models::search::Index,
};

fn main() -> Result<(), failure::Error> {
  let api = XivApi::with_key("d0b5477b34c944e3b87b8a46");

  // let res = api
  //   .character_search()
  //   .name("Duvivi Duvi")
  //   .server(World::Adamantoise)
  //   .send()?;

  // let id = res.characters[0].id;

  let res = api.character(2).send()?;

  println!("{:#?}", res);

  Ok(())
}
