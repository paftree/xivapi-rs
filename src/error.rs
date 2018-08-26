pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
  #[fail(display = "could not url encode params")]
  UrlEncode(serde_urlencoded::ser::Error),
  #[fail(display = "could not process request")]
  Reqwest(reqwest::Error,)
}
