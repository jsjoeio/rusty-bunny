extern crate percent_encoding; 

use rocket::response::Redirect;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};


// Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_twitter_url(query: &str) -> Redirect {
    if query == "tw" {
        Redirect::to("https://twitter.com")

    // Check if it looks like a Twitter profile
    } else if &query[..4] == "tw @" {
        let profile_url = construct_twitter_profile_url(&query[4..]);

        Redirect::to(profile_url)
    } else {
        // Assume the other match is "tw sometext"
        // and search on Twitter
        let twitter_search_url = construct_twitter_search_url(&query[3..]);
        println!("the url is {}", twitter_search_url);

        Redirect::to(twitter_search_url)
    }
}

pub fn construct_twitter_profile_url(profile: &str) -> String {
    format!("https://twitter.com/{}", profile)
}

//TODO write test for this
pub fn construct_twitter_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    format!("https://twitter.com/search?q={}", encoded_query)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_construct_twitter_profile_url() {
      let fake_profile = "jsjoeio";
      assert_eq!(construct_twitter_profile_url(fake_profile), "https://twitter.com/jsjoeio");
  }
}