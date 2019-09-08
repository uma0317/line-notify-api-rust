use std::fmt;

pub struct Response {
	content: reqwest::Response,
}

impl Response {
	pub fn new(content: reqwest::Response) -> Response {
		Response { content }
	}
}
impl fmt::Debug for Response {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(&self.content, f)
	}
}
