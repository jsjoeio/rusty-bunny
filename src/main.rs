#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use] extern crate rocket;
extern crate percent_encoding; // 2.1.0

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use rocket::response::Redirect;
// use rocket::http::RawStr;

// Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
// rename cmd to query
#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    println!("The cmd is: {}", cmd);
    // We need a way to match only on the cmd, without losing the rest of the query 
    // "tw something"
    let command = get_command_from_query_string(&cmd);

    match command {
        "mail" => Redirect::to("https://mail.google.com/"),
        "cal" => Redirect::to("https://calendar.google.com/"),
        "tw" => construct_twitter_url(&cmd),
        "gh" => construct_github_url(&cmd),
        _ => {
            // If no match, we search on Google
            let google_search = construct_google_search_url(&cmd);

            Redirect::to(google_search)
        }
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index, search]).launch();
}

fn construct_twitter_url(query: &str) -> Redirect {
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

fn construct_github_url(query: &str) -> Redirect {
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
fn get_command_from_query_string(query_string: &str) -> &str {
    // If it has a space, we know that it is more than the command
    if query_string.contains(" ") {
        // We need to this to know where to slice the string
        // TODO add note about why we need the unrap_or (tbh i'm not sure why...)
        // copied from StackOverflow (don't have source)
        let index_of_space = query_string.find(" ").unwrap_or(0);
        return &query_string[..index_of_space];
    }

    return query_string;
}

fn construct_twitter_profile_url(profile: &str) -> String {
    format!("https://twitter.com/{}", profile)
}

//TODO write test for this 
fn construct_twitter_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    format!("https://twitter.com/search?q={}", encoded_query)
}

fn construct_google_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    format!("https://google.com/search?q={}", encoded_query)
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_construct_twitter_profile_url() {
      let fake_profile = "jsjoeio";
      assert_eq!(construct_twitter_profile_url(fake_profile), "https://twitter.com/jsjoeio");
  }

  #[test]
  fn test_get_command_from_query_string_no_whitespace() {
      // Test with command only
      let actual = get_command_from_query_string("tw");
      let expected = "tw";
      assert_eq!(actual, expected);
  }

  #[test]
  fn test_get_command_from_query_string_with_whitespace() {
      let actual = get_command_from_query_string("tw @jsjoeio");
      let expected = "tw";
      assert_eq!(actual, expected);
  }
}