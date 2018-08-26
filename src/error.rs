pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
  #[fail(display = "could not url encode params: {}", _0)]
  UrlEncode(#[cause] serde_urlencoded::ser::Error),
  #[fail(display = "could not process request: {}", _0)]
  Reqwest(#[cause] reqwest::Error),
  #[fail(display = "could not parse json: {}", _0)]
  Json(#[cause] serde_json::Error),
  #[fail(display = "the api returned an error: {}", _0)]
  Api(ApiError),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApiError {
  pub error: bool,
  pub message: String,
}

impl std::fmt::Display for ApiError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}
