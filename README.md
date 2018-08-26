# xivapi-rs

It's really basic!

```rust
use xivapi::prelude::*;

fn main() -> Result<()> {
  let api = XivApi::default();

  let search = api
    .search()
    .string("allagan")
    .send()?;

  println!("{:#?}", search.results);

  Ok(())
}
```
