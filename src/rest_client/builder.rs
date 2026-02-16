use super::client::RestClient;
use crate::Result;

#[derive(Debug, Default)]
pub(crate) struct RestClientBuilder {
	/// The url to be used in the request
	url: Option<String>,
	/// Username, defaults to None.
	username: Option<String>,
	/// Password, defaults to None.
	password: Option<String>,
	/// Request timeout, defaults to 75.
	timeout: Option<u32>,
	/// Root for the API requests, defaults to "rest/api"
	api_root: Option<String>,
	/// Version of the API to use, defaults to "latest"
	api_version: Option<String>,
	/// Turn on/off SSL verification, defaults to true
	verify_ssl: Option<bool>,
	/// Inner client session object
	session: Option<reqwest::Client>,
}

impl RestClientBuilder {
	pub fn url(mut self, url: impl Into<String>) -> RestClientBuilder {
		self.url = Some(url.into());
		self
	}

	pub fn username(mut self, username: impl Into<String>) -> RestClientBuilder {
		self.username = Some(username.into());
		self
	}

	pub fn password(mut self, password: impl Into<String>) -> RestClientBuilder {
		self.password = Some(password.into());
		self
	}

	pub fn timeout(mut self, timeout: u32) -> RestClientBuilder {
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

	pub fn verify_ssl(mut self, verify_ssl: bool) -> RestClientBuilder {
		self.verify_ssl = Some(verify_ssl);
		self
	}

	pub fn session(mut self, session: reqwest::Client) -> RestClientBuilder {
		self.session = Some(session);
		self
	}

	pub fn build(self) -> Result<RestClient> {
		Ok(RestClient {
			url: url::Url::parse(&self.url.unwrap_or_default())?,
			username: self.username,
			password: self.password,
			timeout: self.timeout.unwrap_or(75),
			api_root: self.api_root.unwrap_or("rest/api".to_string()),
			api_version: self.api_version.unwrap_or("latest".to_string()),
			verify_ssl: self.verify_ssl.unwrap_or(true),
			session: self.session.unwrap_or(reqwest::Client::new()),
		})
	}
}