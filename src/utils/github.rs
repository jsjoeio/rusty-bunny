extern crate percent_encoding; 

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_github_url(query: &str) -> String {
    if query == "gh" {
        let github_dotcom = "https://github.com";
        
        github_dotcom.to_string()
    } else {
        // Assume the other match is "gh page"
        // optimistic, eh?
        let encoded_query = utf8_percent_encode(&query[3..], FRAGMENT).to_string();
        let github_url = format!("https://github.com/{}", encoded_query);

        github_url
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_construct_github_profile_url_with_gh() {
      let fake_query = "gh";
      assert_eq!(construct_github_url(fake_query), "https://github.com");
  }

  #[test]
  fn test_construct_github_profile_url_with_repo_url() {
      let fake_query = "gh jsjoeio/rusty-bunny";
      assert_eq!(construct_github_url(fake_query), "https://github.com/jsjoeio/rusty-bunny");
  }
}