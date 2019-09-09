use std::fmt;
pub struct Response {
	content: reqwest::r#async::Response
}

impl Response {
    pub fn new(content: reqwest::r#async::Response) -> Response {
        Response { content }
    }

    pub fn content(self) -> reqwest::r#async::Response {
        self.content
    }
}
impl fmt::Debug for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.content, f)
    }
}