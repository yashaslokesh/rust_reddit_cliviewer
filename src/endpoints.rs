use std::collections::HashMap

use lazy_static::lazy_static;

mod client;

// Started writing a lazy_static hashmap, 
// But i realized that match statements are faster
lazy_static {
    static ENDPOINTS: HashMap<&str, &str> =
        [("Hello", "Test"),]
        .iter().cloned().collect();
}

type EndpointDescriptor = &str;

impl RedditClient {
    fn get_endpoint_url(descriptor: EndpointDescriptor) -> &str {
        match descriptor {
            "me" => "api/v1/me",
            "hot" => "hot"
        }
    }
}