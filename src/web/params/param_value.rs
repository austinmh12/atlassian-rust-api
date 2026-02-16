use std::borrow::Cow;

use crate::Result;

pub trait ParamValue<'a> {
	fn as_value(&self) -> Cow<'a, str>;
}

impl ParamValue<'static> for bool {
	fn as_value(&self) -> Cow<'static, str> {
		(*self).to_string().into()
	}
}

impl<'a> ParamValue<'a> for &'a str {
	fn as_value(&self) -> Cow<'a, str> {
		(*self).into()
	}
}

impl ParamValue<'static> for String {
	fn as_value(&self) -> Cow<'static, str> {
		self.clone().into()
	}
}

impl<'a> ParamValue<'a> for &'a String {
	fn as_value(&self) -> Cow<'a, str> {
		(*self).into()
	}
}

impl<'a> ParamValue<'a> for Cow<'a, str> {
	fn as_value(&self) -> Cow<'a, str> {
		self.clone()
	}
}

impl<'a, 'b: 'a> ParamValue<'a> for &'b Cow<'a, str> {
	fn as_value(&self) -> Cow<'a, str> {
		(*self).clone()
	}
}

impl ParamValue<'static> for u32 {
	fn as_value(&self) -> Cow<'static, str> {
		self.to_string().into()
	}
}

impl ParamValue<'static> for u64 {
	fn as_value(&self) -> Cow<'static, str> {
		self.to_string().into()
	}
}

impl ParamValue<'static> for i32 {
	fn as_value(&self) -> Cow<'static, str> {
		self.to_string().into()
	}
}

impl ParamValue<'static> for i64 {
	fn as_value(&self) -> Cow<'static, str> {
		self.to_string().into()
	}
}

impl ParamValue<'static> for f32 {
	fn as_value(&self) -> Cow<'static, str> {
		self.to_string().into()
	}
}

impl ParamValue<'static> for f64 {
	fn as_value(&self) -> Cow<'static, str> {
		self.to_string().into()
	}
}