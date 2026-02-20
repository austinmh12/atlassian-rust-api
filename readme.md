# Atlassian Rust API
atlassian-rust-api is an async wrapper for the Atlassian REST API. It provides a simple, builder-pattern focused way to interact with the Atlassian products. It is based on the official REST APIs for each product.

Note that this is currently under heavy construction and I am currently focusing on Jira Data Center/Cloud for the moment. JSM and Confluence are in the pipeline right after, followed by the rest of the tools.

## Versions
- Jira Data Center: v9.17.0

# Features
## Cargo Feature Flags
- `jira`: Add access to the `jira` crate.
- `experimental`: Add access to experimental endpoints.

# Usage
See the `examples/` folder for more in-depth usage.

## Quickstart
These quickstarts assume `tokio` is being used to provide the async runtime, but the library is agnostic of the async runtime.

### Jira
```rust
use atlassian_rust_api::Jira;

#[tokio::main]
async fn main() -> atlassian_rust_api::Result<()> {
	let jira = Jira::builder()
		.url("https://jira.example.com")
		.username("user")
		.password("password")
		.build()?; // Errs if the URL cannot be parsed.
	let issue = jira.get_issue("ABC-123").send().await?;
	if let Some(fields) = issue.get("fields") {
		println!("{:?}", fields);
	}

	Ok(())
}
```

# Design Pattern
## Endpoints
Every endpoint is made up of 3 parts, the `EndpointBuilder`, the `EndpointRequest`, and the `impl Client` block in the endpoint. Each endpoint to one of the core REST API endpoints gets each of these things.

### EndpointBuilder
The `EndpointBuilder` is what the user interacts with and holds a copy of the `RestClient` for making the call to the REST API and the `EndpointRequest`. The builder implements the setters for the fields contained in the `EndpointRequest` as well as the `send()` function for executing the request.

```rust
pub struct EndpointBuilder {
	client: Arc<RestClient>,
	request: EndpointRequest,
}

impl EndpointBuilder {
	// For the impl Client block to create this
	fn new(client: Arc<RestClient>) -> EndpointBuilder {
		EndpointBuilder { client, request: EndpointRequest::default() }
	}

	// Required fields are for the impl Client block and it forces them to be set with
	// the function it defines to use this builder with
	fn required(mut self, required: i64) -> EndpointBuilder {
		self.request.required = required;
		self
	}

	// Public fields are for the user to set if they want
	pub fn optional(mut self, optional: impl Into<String>) -> EndpointBuilder {
		self.request.optional = Some(optional.into());
		self
	}

	// The actual sending of the request to the REST API
	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}
```

### EndpointRequest
The `EndpointRequest` holds the information required to make the request to the REST API. It also implements the `Endpoint` trait which builds the URL, the query parameters, and the body of the request. It is separate from the `EndpointBuilder` so that each of the fields can remain private from the user and so that it can implement `Default`

```rust
#[derive(Default)]
struct EndpointRequest {
	required: i64,
	optional: Option<String>,
}

impl Endpoint for EndpointRequest {
	fn endpoint(&self) -> Cow<'static, str> {
		format!("resource/{}", self.required).into()
	}

	fn parameters(&self) -> QueryParams<'_> {
		let mut params = QueryParams::default();
		params.push_opt("optionalField", self.optional);
		params
	}
}
```

### impl Client Block
The `impl Client` block is created in the same file as the `EndpointBuilder` and `EndpointRequest` so that they don't have to be explicitly publicized. The user is responsible for setting required fields in the initial call and the `Client` fn should always return the `EndpointBuilder` regardless if there are zero fields for the endpoint. The user should be responsible for calling `.send().await?;` on the `EndpointBuilder` to execute the request.

```rust
impl Client {
	pub fn get_endpoint(&self, required: i64) -> EndpointBuilder {
		EndpointBuilder::new(Arc::clone(&self.client)).required(required)
	}
}
```