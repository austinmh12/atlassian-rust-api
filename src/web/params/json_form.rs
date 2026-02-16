use serde::Serialize;
use serde_json::{Map, json};

use crate::Result;

#[derive(Debug, Clone, Default)]
pub struct JsonFormParams {
	params: serde_json::Value,
}

impl JsonFormParams {
	pub fn push<V>(&mut self, key: &str, value: V) -> &mut Self
	where
		V: Serialize,
	{
		let mut default_map = Map::default();
		let map = self.params.as_object_mut().unwrap_or(&mut default_map);
		map.insert(key.into(), json!(value));
		self.params = json!(map);
		self
	}

	pub fn push_opt<V>(&mut self, key: &str, value: Option<V>) -> &mut Self
	where
		V: Serialize,
	{
		let mut default_map = Map::default();
		let map = self.params.as_object_mut().unwrap_or(&mut default_map);
		if let Some(value) = value {
			map.insert(key.into(), json!(value));
			self.params = json!(map);
		}
		self
	}

	pub fn into_body(self) -> Result<Option<(&'static str, Vec<u8>)>> {
		let body = serde_json::to_vec(&self.params)?;
		Ok(Some((
			"application/json",
			body
		)))
	}
}