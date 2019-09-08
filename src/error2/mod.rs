pub mod notify;

pub struct Error {
	inner: reqwest::Error
}

struct Inner {
	kind: Kind,
	url: Option<Url>,
}

pub enum Kind {
	OK,
	BadRequest,
	Unauthorized,
	InternalServerError,
	Other,
}