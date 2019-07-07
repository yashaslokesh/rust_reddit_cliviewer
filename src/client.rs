// extern crate reqwest;
// extern crate rand;
// extern crate webbrowser;

use std::collections::HashMap;
// use std::error::Error;
// use std::fs::File;
// use std::io::prelude::*;
// use std::path::Path;

use rand::distributions::Alphanumeric;
use rand::{Rng};

use url::Url;

use webbrowser::{self, Browser};

// contain the "state" param
pub struct RedditClient {
    client_id: String,
    response_type: String,
    state: String,
    redirect_uri: String,
    duration: String,
    scope: String,
}

impl RedditClient {
    pub fn new() -> RedditClient {
        let client_state: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .collect();

        let client_scope = String::from("identity mysubreddits vote save");

        let client = RedditClient {
            client_id: String::from("2_ibU4iR5cg7Sg"),
            response_type: String::from("code"),
            state: client_state,
            redirect_uri: String::from("https://www.google.com"),
            duration: String::from("permanent"),
            scope: client_scope,
        };

        client
    }

    pub fn redirect_user_for_auth(&self) {
        let url = format!("https://www.reddit.com/api/v1/authorize?client_id={}&response_type={}&state={}&redirect_uri={}&duration={}&scope={}",
       self.client_id, self.response_type, &self.state, self.redirect_uri, self.duration, self.scope);

        if webbrowser::open_browser(Browser::Safari, &url).is_ok() {
            println!("Redirected user to Reddit login successfully",);
        }
    }

    // Returns true if log in was successful
    pub fn process_redirect_url(&self, url: &str) -> bool {
        let parsed = Url::parse(url).unwrap();

        // Contains "state" param always, so you can check if redirect result is valid
        // Contains "error" according to conditions listed on this page:
        // https://github.com/reddit-archive/reddit/wiki/oauth2
        // Else, contains "code" that can be exchanged for a bearer token
        let query_params: HashMap<_, _> = parsed.query_pairs().into_owned().collect();

        let state_param = String::from("state");
        let error_param = String::from("error");

        match query_params.get(&state_param) {
            // If no state returned, the url cannot be valid
            None => false,
            Some(s) => {
                match query_params.get(&error_param) {
                    // No error, so just ensure states match
                    None => s == &self.state,
                    // If there is an error, connection wasn't successful
                    Some(_) => false,
                }
            }
        }
    }
}
