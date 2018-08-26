use xivapi::{
  prelude::*,
  models::search::Index,
};

fn main() {
  let api = XivApi::new();

  let res = api
    .search()
    .index(Index::InstanceContent)
    .string("a")
    .send();

  println!("{:#?}", res);
}
