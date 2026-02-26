use super::client::RestClient;
use crate::Result;

#[derive(Debug, Default)]
pub(crate) struct RestClientBuilder {
	/// The url to be used in the request
	url: Option<url::Url>,
	/// Username, defaults to None.
	username: Option<String>,
	/// Password, defaults to None.
	password: Option<String>,
	/// Request timeout, defaults to 75.
	timeout: Option<u64>,
	/// Root for the API requests, defaults to "rest/api"
	api_root: Option<String>,
	/// Version of the API to use, defaults to "latest"
	api_version: Option<String>,
	/// Inner client session object
	session: Option<reqwest::Client>,
}

impl RestClientBuilder {
	pub fn url(mut self, url: impl Into<String>) -> Result<RestClientBuilder> {
		let url = url.into();
		let url = url::Url::parse(&url)?;
		self.url = Some(url);
		Ok(self)
	}

	pub fn username(mut self, username: impl Into<String>) -> RestClientBuilder {
		self.username = Some(username.into());
		self
	}

	pub fn password(mut self, password: impl Into<String>) -> RestClientBuilder {
		self.password = Some(password.into());
		self
	}

	pub fn timeout(mut self, timeout: u64) -> RestClientBuilder {
		self.timeout = Some(timeout);
		self
	}

	pub fn api_root(mut self, api_root: impl Into<String>) -> RestClientBuilder {
		self.api_root = Some(api_root.into());
		self
	}

	pub fn api_version(mut self, api_version: impl Into<String>) -> RestClientBuilder {
		self.api_version = Some(api_version.into());
		self
	}

	pub fn session(mut self, session: reqwest::Client) -> RestClientBuilder {
		self.session = Some(session);
		self
	}

	pub fn build(self) -> Result<RestClient> {
		Ok(RestClient {
			url: self.url.unwrap(), // Will panic if None
			username: self.username,
			password: self.password,
			timeout: self.timeout.unwrap_or(75),
			api_root: self.api_root.unwrap_or("rest/api".to_string()),
			api_version: self.api_version.unwrap_or("latest".to_string()),
			session: self.session.unwrap_or(reqwest::Client::new()),
		})
	}
}