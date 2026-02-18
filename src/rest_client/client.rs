use reqwest::{Method, Url};
use crate::{Result, Error, web::Endpoint};

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
	/// Turn on/off cloud methods
	pub(crate) cloud: bool,
	/// Inner client session object
	pub(crate) session: reqwest::Client,
}

impl RestClient {
	fn rest_endpoint(&self, path: &str) -> Result<Url> {
		let resource = vec![&self.api_root, &self.api_version, path];
		// Remove leading and trailing '/' from each portion of the resource
		let resource = resource.iter().map(|s| s.trim_matches('/')).collect::<Vec<&str>>().join("/");
		Ok(self.url.join(&resource)?)
	}

	/// Creates a `reqwest::Request` with the given method, sends the request,
	/// and attempts to deserialize the response into the given `T`
	async fn request<T, E>(&self, request: E, method: Method) -> Result<T>
	where
		T: serde::de::DeserializeOwned,
		E: Endpoint,
	{
		let mut url = self.rest_endpoint(&request.endpoint())?;
		request.parameters().add_to_url(&mut url);
		let req = match method {
			Method::GET => self.session.get(url),
			Method::POST => self.session.post(url),
			Method::PATCH => self.session.patch(url),
			Method::PUT => self.session.put(url),
			Method::DELETE => self.session.delete(url),
			_ => return Err(Error::UnsupportedOperation(method))
		}.header("Accept", "application/json");
		let req = if let Some((mime_type, body)) = request.body()? {
			req.header("Content-Type", mime_type).body(body)
		} else {
			req.header("Content-Type", "application/json") // Maybe...
		};
		let req = req.basic_auth(self.username.as_ref().unwrap(), self.password.as_ref()); // Hard code for now and panic if not provided
		let resp = req.send().await?;
		match resp.error_for_status_ref() {
			Ok(_) => Ok(resp.json().await?),
			Err(e) => Err(e.into())
		}
	}

	/// Creates a `reqwest::Request` with the given method, sends the request,
	/// and returns nothing if the request is successful.
	async fn ignore<E>(&self, request: E, method: Method) -> Result<()>
	where
		E: Endpoint,
	{
		let mut url = self.rest_endpoint(&request.endpoint())?;
		request.parameters().add_to_url(&mut url);
		let req = match method {
			Method::GET => self.session.get(url),
			Method::POST => self.session.post(url),
			Method::PATCH => self.session.patch(url),
			Method::PUT => self.session.put(url),
			Method::DELETE => self.session.delete(url),
			_ => return Err(Error::UnsupportedOperation(method))
		}.header("Accept", "application/json");
		let req = if let Some((mime_type, body)) = request.body()? {
			req.header("Content-Type", mime_type).body(body)
		} else {
			req.header("Content-Type", "application/json") // Maybe...
		};
		let req = req.basic_auth(self.username.as_ref().unwrap(), self.password.as_ref()); // Hard code for now and panic if not provided
		let resp = req.send().await?;
		match resp.error_for_status_ref() {
			Ok(_) => Ok(()),
			Err(e) => Err(e.into())
		}
	}

	/// Convenience function for GET requests.
	pub async fn get<T, E>(&self, request: E) -> Result<T>
	where
		T: serde::de::DeserializeOwned,
		E: Endpoint,
	{
		self.request(request, Method::GET).await
	}

	/// Convenience function for PUT requests with no return.
	pub async fn put_ignore<E>(&self, request: E) -> Result<()>
	where
		E: Endpoint,
	{
		self.ignore(request, Method::PUT).await
	}
}