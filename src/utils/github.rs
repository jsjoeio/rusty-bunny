extern crate percent_encoding; 

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_github_url(query: &str) -> String {
    if query == "gh" {
        let github_dotcom = "https://github.com";
        
        return github_dotcom.to_string(); 
    } else {
        // Assume the other match is "gh page"
        // optimistic, eh?
        let encoded_query = utf8_percent_encode(&query[3..], FRAGMENT).to_string();
        let github_url = format!("https://github.com/{}", encoded_query);

        return github_url;
    }
}