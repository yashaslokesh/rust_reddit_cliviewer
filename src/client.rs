use base64::encode;

use rand::distributions::Alphanumeric;
use rand::Rng;

use serde_json::Value;

use std::collections::HashMap;
use std::str;
// use std::error::Error;
// use std::fs::File;
// use std::io::prelude::*;
// use std::path::Path;

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
    // Don't have acesss to these things at creation
    access_token: Option<String>,
    refresh_token: Option<String>,
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
            access_token: None,
            refresh_token: None,
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
    pub fn parse_redirect_url(&mut self, url: &str) -> bool {
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
                    None => {
                        if s == &self.state {
                            let code_param = String::from("code");
                            self.retrieve_access_and_refresh_tokens(
                                query_params.get(&code_param).unwrap(),
                            );
                            true
                        } else {
                            false
                        }
                    }
                    // If there is an error, connection wasn't successful
                    Some(_) => false,
                }
            }
        }
    }

    // Retrieves and sets access_token and refresh_token fields in
    // the respective fields, should be called once
    fn retrieve_access_and_refresh_tokens(&mut self, code: &str) {
        let url = "https://www.reddit.com/api/v1/access_token";

        let client = reqwest::Client::new();

        let user = &self.client_id;
        // Empty since this will be an installed app
        let pass = "";

        // let s = b"Aladdin:open sesame";
        let user_pass = format!("{}:{}", user, pass);
        let encoded = encode(&user_pass);

        let auth_string = format!("Basic {}", encoded);

        let body = format!(
            "grant_type=authorization_code&code={}&redirect_uri={}",
            code, &self.redirect_uri
        );

        let result = client
            .post(url)
            .header("Authorization", auth_string)
            .body(body)
            .send();

        let response_text = result.unwrap().text().unwrap();

        print!("Result: {}", &response_text);

        let v: Value = serde_json::from_str(&response_text).unwrap();

        print!("Value: {}", v);

        self.access_token = Some(String::from(
            v.get("access_token").unwrap().as_str().unwrap(),
        ));

        self.refresh_token = Some(String::from(
            v.get("refresh_token").unwrap().as_str().unwrap(),
        ))
    }
}
