use xivapi::{XivApi, builder::Builder};

fn main() {
  let api = XivApi::new();
  println!("{:#?}", api.search().string("allagan").send());
}
