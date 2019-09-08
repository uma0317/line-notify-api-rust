extern crate line_notify_api_rust as line;

use super::common;

#[test]

fn authorize_url() {
	let client = common::create_client();
	let url = client.authorize_url();
	assert_eq!("https://notify-bot.line.me/oauth/authorize?response_type=code&client_id=BH1ZrONVRI0DZnHg1spZVU&redirect_uri=http://localhost:8080/oauth&scope=notify&state=aaa&response_mode=form_post", url);
	// assert_eq!("200", format!("{:?}", res.unwrap().content().status()))
}
