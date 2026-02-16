use reqwest::Url;
use crate::Result;

#[derive(Debug, Clone)]
pub(crate) struct RestClient {
	/// The url to be used in the request
	pub(crate) url: Url,
	/// Username, defaults to None.
	pub(crate) username: Option<String>,
	/// Password, defaults to None.
	pub(crate) password: Option<String>,
	/// Request timeout, defaults to 75.
	pub(crate) timeout: u32,
	/// Root for the API requests, defaults to "rest/api"
	pub(crate) api_root: String,
	/// Version of the API to use, defaults to "latest"
	pub(crate) api_version: String,
	/// Turn on/off SSL verification, defaults to true
	pub(crate) verify_ssl: bool,
	/// Inner client session object
	pub(crate) session: reqwest::Client,
}

impl RestClient {
	fn rest_endpoint(&self, path: &str) -> Result<Url> {
		Ok(self.url.join(&self.api_root)?.join(&self.api_version)?.join(path)?)
	}
}