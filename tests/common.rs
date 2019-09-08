use line_notify_api_rust as line;

use line::Client;
pub fn create_client() -> Client {
	Client::new(
		"ZtX1PFSSDxgWC1Mtueys1ri5KdKGtzWF8B2DNnqmlUe",
		"BH1ZrONVRI0DZnHg1spZVU",
		"Km1xRtjJyINpaSllulRb6gpUykZW6OApZnyCzOz9WEb",
		"http://localhost:8080/oauth",
	)
}
