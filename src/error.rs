use std::fmt;

pub struct Error {
	content: reqwest::Error,
}

impl Error {
	pub fn new(content: reqwest::Error) -> Error {
		Error { content }
	}
}

impl fmt::Debug for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if let Some(ref url) = self.content.url() {
			f.debug_tuple("Error")
				.field(&self.content.get_ref())
				.field(url)
				.finish()
		} else {
			f.debug_tuple("Error")
				.field(&self.content.get_ref())
				.finish()
		}
	}
}

pub type Result<T> = std::result::Result<T, Error>;
