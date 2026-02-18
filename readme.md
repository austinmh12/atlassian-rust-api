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
	let issue = jira.get_issue("ABC-123").await?;
	if let Some(fields) = issue.get("fields") {
		println!("{:?}", fields);
	}

	Ok(())
}
```