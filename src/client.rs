use crate::Error;
use crate::Response;
use crate::Result;

pub struct Client {
	token: String,
}

impl Client {
	pub fn new(token: &str) -> Client {
		Client {
			token: token.to_owned(),
		}
	}

	pub fn notify(self, message: &str) -> Result<Response> {
		let res = reqwest::Client::new()
			.post("https://notify-api.line.me/api/notify")
			.bearer_auth(self.token)
			.form(&[("message", message)])
			.send();

		match res {
			Ok(n) => Ok(Response::new(n)),
			Err(e) => Err(Error::new(e)),
		}
	}

	pub fn status(self) -> Result<Response> {
		let res = reqwest::Client::new()
			.get("https://notify-api.line.me/api/status")
			.bearer_auth(self.token)
			.send();
			
		match res {
			Ok(n) => Ok(Response::new(n)),
			Err(e) => Err(Error::new(e)),
		}
	}
}
