use std::borrow::Cow;
use url::Url;

use crate::web::params::param_value::ParamValue;


#[derive(Debug, Clone, Default)]
pub struct QueryParams<'a> {
	params: Vec<(Cow<'a, str>, Cow<'a, str>)>,
}

impl<'a> QueryParams<'a> {
	pub fn push<'b, K, V>(&mut self, key: K, value: V) -> &mut Self
	where
		'b: 'a,
		K: Into<Cow<'a, str>>,
		V: ParamValue<'b>,
	{
		self.params.push((key.into(), value.as_value()));
		self
	}

	pub fn push_opt<'b, K, V>(&mut self, key: K, value: Option<V>) -> &mut Self
	where
		'b: 'a,
		K: Into<Cow<'a, str>>,
		V: ParamValue<'b>,
	{
		if let Some(value) = value {
			self.params.push((key.into(), value.as_value()));
		}
		self
	}

	pub fn add_to_url(&self, url: &mut Url) {
		if self.params.len() == 0 {
			return
		}
		let mut pairs = url.query_pairs_mut();
		pairs.extend_pairs(self.params.iter());
	}
}