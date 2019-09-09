use crate::parameter::NotifyParam;
use crate::Error;
use crate::Response;
use crate::Result;

use futures::Future;

pub struct Client {
	token: String,
	id: String,
	secret: String,
	redirect_uri: String,
}

impl Client {
	pub fn new(token: &str, id: &str, secret: &str, redirect_uri: &str) -> Client {
		Client {
			token: token.to_owned(),
			id: id.to_owned(),
			secret: secret.to_owned(),
			redirect_uri: redirect_uri.to_owned(),
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

	// pub fn notify_with_param(self, param: NotifyParam) -> Result<Response> {
	// }

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

	pub fn revoke(self) -> Result<Response> {
		let res = reqwest::Client::new()
			.post("https://notify-api.line.me/api/revoke")
			.bearer_auth(self.token)
			.send();

		match res {
			Ok(n) => Ok(Response::new(n)),
			Err(e) => Err(Error::new(e)),
		}
	}

	pub fn authorize(self) -> Result<Response> {
		let res = reqwest::Client::new()
			.get("https://notify-bot.line.me/oauth/authorize")
			.query(&[
				("response_type", "code"),
				("client_id", &self.id),
				("redirect_uri", "http://localhost:8080/oauthpost"),
				("scope", "notify"),
				("state", "aaa"),
				("response_mode", "form_post"),
			])
			.send();

		match res {
			Ok(n) => Ok(Response::new(n)),
			Err(e) => Err(Error::new(e)),
		}
	}

	//todo urlencode is too bad
	pub fn authorize_url(self) -> String {
		let mut url = format!("https://notify-bot.line.me/oauth/authorize?response_type=code&client_id={id}&redirect_uri={uri}&scope=notify&state=aaa&response_mode=form_post", id=&self.id, uri=&self.redirect_uri);

		url
	}

	pub fn token(self, code: &str) -> Result<Response> {
		let url = format!("https://notify-bot.line.me/oauth/token?grant_type=authorization_code&code={code}&redirect_uri={uri}&client_id={id}&client_secret={secret}", code=code, uri=&self.redirect_uri, id=&self.id, secret=&self.secret);
		let res = reqwest::Client::new().post(&url).send();
		// let res = reqwest::Client::new()
		// 	.post("https://notify-bot.line.me/oauth/token")
		// 	.query(&[
		// 		("grant_type", "authorization_code"),
		// 		("code", code),
		// 		("redirect_uri", "http://localhost:8080/oauthpost"),
		// 		("client_id", &self.id),
		// 		("client_secret", &self.secret),
		// 	])
		// 	.send();

		match res {
			Ok(n) => Ok(Response::new(n)),
			Err(e) => Err(Error::new(e)),
		}
	}

	pub fn async_token(
		self,
		code: &str,
	) -> Box<impl Future<Item = crate::r#async::Response, Error = Error>> {
		let url = format!("https://notify-bot.line.me/oauth/token?grant_type=authorization_code&code={code}&redirect_uri={uri}&client_id={id}&client_secret={secret}", code=code, uri=&self.redirect_uri, id=&self.id, secret=&self.secret);

		Box::new(
			reqwest::r#async::Client::new()
				.post(&url)
				.send()
				.and_then(move |res| {
					println!("{:?}", res);
					Ok(crate::r#async::Response::new(res))
				})
				// .and_then(move |res| Ok(crate::r#async::Response::new(res)))
				.or_else(move |res| {
					eprintln!("{}", res);
					Err(Error::new(res))
				}),
		)
	}
}
