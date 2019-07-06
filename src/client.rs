// extern crate reqwest;
// extern crate rand;
// extern crate webbrowser;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

use url::Url;

use webbrowser::{self, Browser};

pub fn test_reqwest() {
    // let mut req = reqwest::get("https://www.rust-lang.org").unwrap();
    // let body = req.text().unwrap();

    // // Create a path to the desired file
    // let path = Path::new("hello.txt");
    // let display = path.display();

    // // Open the path in read-only mode, returns `io::Result<File>`
    // let mut file = match File::open(&path) {
    //     // The `description` method of `io::Error` returns a string that
    //     // describes the error
    //     Err(why) => panic!("couldn't open {}: {}", display,
    //                                                why.description()),
    //     Ok(file) => file,
    // };

    // // Read the file contents into a string, returns `io::Result<usize>`
    // let mut s = String::new();
    // match file.read_to_string(&mut s) {
    //     Err(why) => panic!("couldn't read {}: {}", display,
    //                                                why.description()),
    //     Ok(_) => print!("{} contains:\n{}", display, s),
    // }

    // // `file` goes out of scope, and the "hello.txt" file gets closed

    // let out_path = Path::new("out/rust-lang-org.txt");
    // let out_display = out_path.display();

    //     // Open a file in write-only mode, returns `io::Result<File>`
    // let mut file = match File::create(&out_path) {
    //     Err(why) => panic!("couldn't create {}: {}", out_display, why.description()),
    //     Ok(file) => file,
    // };

    // // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    // match file.write_all(body.as_bytes()) {
    //     Err(why) => panic!("couldn't write to {}: {}", out_display, why.description()),
    //     Ok(_) => println!("successfully wrote to {}", out_display),
    // }



    // println!("body = {:?}", body);
}

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

        if webbrowser::open_browser(webbrowser::Browser::Safari, &url).is_ok() {
            println!("Redirected user to Reddit login successfully", );
        }
    }

    pub fn process_redirect_url(&self, url: &str) -> bool {
       let parsed = Url::parse(url).unwrap(); 

        // Contains "state" param always, so you can check if redirect result is valid
        // Contains "error" according to conditions listed on this page:
        // https://github.com/reddit-archive/reddit/wiki/oauth2
        // Else, contains "code" that can be exchanged for a bearer token
        let query_params: HashMap<_,_> = parsed.query_pairs().into_owned()
                                        .collect();
        
        let state_param = String::from("state");

        match query_params.get(&state_param) {
            None => false,
            Some(s) => s == &self.state,
        }

        // assert!(query_params.get(&state_param).unwrap() == self.state);
    }
}

// Make method of RedditClient
pub fn process_redirect_url(url: &str) {
    let parsed = Url::parse(url).unwrap();
    // Contains "state" param always, so you can check if redirect result is valid
    // Contains "error" according to conditions listed on this page:
    // https://github.com/reddit-archive/reddit/wiki/oauth2
    // Else, contains "code" that can be exchanged for a bearer token
    let query_params: HashMap<_,_> = parsed.query_pairs().into_owned()
                                        .collect();

}

// Make part of RedditClient
pub fn redirect_user_for_auth() {
    // From reddit apps website
    let client_id = "2_ibU4iR5cg7Sg";

    // Change to "token" for implicit grants
    let response_type = "code";

    // Generates random string of length 64 to authenticate the redirect
    let state: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .collect();

    // Testing
    // println!("State string: {}", state);

    // Temporary
    let redirect_uri = "https://www.google.com";

    // Allows us to keep users logged in FOREVER
    let duration = "permanent";

    // Add to or remove from to change access levels
    let scope = "identity mysubreddits vote save";

    let url = format!("https://www.reddit.com/api/v1/authorize?client_id={}&response_type={}&state={}&redirect_uri={}&duration={}&scope={}",
    client_id, response_type, &state, redirect_uri, duration, scope);

    if webbrowser::open_browser(webbrowser::Browser::Safari, &url).is_ok() {
        println!("Redirected user to Reddit login successfully", );
    }
}