extern crate percent_encoding; 

use rocket::response::Redirect;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_github_url(query: &str) -> Redirect {
    if query == "gh" {
        Redirect::to("https://github.com")

    } else {
        // Assume the other match is "gh page"
        // optimistic, eh?
        let encoded_query = utf8_percent_encode(&query[3..], FRAGMENT).to_string();
        let github_url = format!("https://github.com/{}", encoded_query);

        Redirect::to(github_url)
    }
}