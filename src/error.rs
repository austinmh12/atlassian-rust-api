use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
	#[from]
	URLParseError(url::ParseError),
	#[from]
	ApiResponseError(reqwest::Error),
	#[from]
	JSONParseError(serde_json::Error),
	UnsupportedOperation(reqwest::Method),
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{self:?}")
	}
}

impl std::error::Error for Error {}