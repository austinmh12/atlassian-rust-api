use std::sync::Arc;

use crate::rest_client::{RestClient, RestClientBuilder};
use crate::Result;

/// A Jira REST API client wrapper. Allows for interacting with all of the Jira REST API
/// endpoints
/// 
/// ## Example
/// ```no_run
/// use atlassian_rest_api::Jira;
/// 
/// let jira = Jira::new("http://localhost.com")? // Will return an error if URL cannot be parsed
/// 	.username("username")
/// 	.password("password")
/// 	.build();
/// let issues = jira.search("project = PROJ and issuetype = Bug").send().await?;
/// ```
#[derive(Debug, Clone)]
pub struct Jira {
	pub(crate) client: Arc<RestClient>,
}

impl Jira {
	/// Create a new Jira client with a given URL.
	pub fn new(url: impl Into<String>) -> Result<JiraBuilder> {
		JiraBuilder::default().url(url)
	}
}

#[derive(Debug, Default)]
pub struct JiraBuilder {
	client: RestClientBuilder
}

impl JiraBuilder {
	/// The URL of the Jira site to connect to.
	fn url(mut self, url: impl Into<String>) -> Result<JiraBuilder> {
		self.client = self.client.url(url)?;
		Ok(self)
	}

	/// The username of the account to connect with.
	pub fn username(mut self, username: impl Into<String>) -> JiraBuilder {
		self.client = self.client.username(username.into());
		self
	}

	/// The password of the account to connect with.
	pub fn password(mut self, password: impl Into<String>) -> JiraBuilder {
		self.client = self.client.password(password.into());
		self
	}

	/// Timeout duration for requests in seconds. Defaults to `75`.
	pub fn timeout(mut self, timeout: u64) -> JiraBuilder {
		self.client = self.client.timeout(timeout);
		self
	}

	/// The API root. Defaults to `rest/api`.  
	pub fn api_root(mut self, api_root: impl Into<String>) -> JiraBuilder {
		self.client = self.client.api_root(api_root.into());
		self
	}

	/// The API version. Defaults to `latest`.
	pub fn api_version(mut self, api_version: impl Into<String>) -> JiraBuilder {
		self.client = self.client.api_version(api_version.into());
		self
	}

	/// Pass an existing reqwest Client if one is already established.
	pub fn session(mut self, session: reqwest::Client) -> JiraBuilder {
		self.client = self.client.session(session);
		self
	}

	/// Build the [Jira] client
	pub fn build(self) -> Result<Jira> {
		let client = self.client.build()?;
		Ok(Jira {
			client: Arc::new(client)
		})
	}
}