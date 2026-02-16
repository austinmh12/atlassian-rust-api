use std::borrow::Cow;

use crate::Result;
use super::QueryParams;

pub trait Endpoint {
	/// The path to the endpoint
	fn endpoint(&self) -> Cow<'static, str>;

	/// Query parameters for the endpoint
	fn parameters(&self) -> QueryParams<'_> {
		QueryParams::default()
	}

	/// The body of the endpoint
	fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>> {
		Ok(None)
	}
}