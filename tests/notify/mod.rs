extern crate line_notify_api_rust as line;

use super::common;

#[test]
fn notify() {
	let client = common::create_client();
	let res = client.notify("test");
	assert_eq!("200", format!("{:?}", res.unwrap().content().status()))
}

#[test]
fn status() {
	let client = common::create_client();
	let res = client.status();
	assert_eq!("200", format!("{:?}", res.unwrap().content().status()))
}

#[test]

fn token() {
	let client = common::create_client();
	let res = client.token();
	println!("{:?}", res.unwrap().content().text());
	// assert_eq!("200", format!("{:?}", res.unwrap().content().status()))
}
