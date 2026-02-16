/// Macro that allows for calling `.await` on a struct to turn the following
/// ```rust
/// let results: T = client.endpoint().send().await?;
/// ```
/// into
/// ```rust
/// let results: T = client.endpoint().await?;
/// ```
macro_rules! futurize {
	(&str:ident) => {
		impl IntoFuture for &str {
			type Output = Result<()>;
			type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output>>>;

			fn into_future(self) -> Self::IntoFuture {
				Box::pin(self.send())
			}
		}
	};
	($str:ident, $out:ty) => {
		impl IntoFuture for &str {
			type Output = Result<$out>;
			type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output>>>;

			fn into_future(self) -> Self::IntoFuture {
				Box::pin(self.send())
			}
		}
	}
}