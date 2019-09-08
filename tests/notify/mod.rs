extern crate line_notify_api_rust as line;

use super::common;

#[test]
fn notify() {
	let client = common::create_client();
	println!("{:?}", client.notify("test"));
}
