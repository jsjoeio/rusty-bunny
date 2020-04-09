extern crate percent_encoding; 

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_google_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let google_search_url = format!("https://google.com/search?q={}", encoded_query);

    google_search_url
}