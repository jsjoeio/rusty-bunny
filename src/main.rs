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
        _ => Redirect::to("https://google.com/")
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index, search]).launch();
}

// Write a function to construct twitter url 
// then maybe a function to construct twitter profile url
// and another to construct twitter search 

fn construct_twitter_url(query: &str) -> Redirect {
    // check the query
    // if it's only the cmd "tw"
    // do some string matching
    // matches "tw "
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
    // matches "tw @"
    // matches tw fsafdas"
    // default "twitter.com"
    // send them to Twitter.com
    // if it contains the @symbol 
    // send them to that twitter profile
    // if it contains text but no @ symbol
    // send them to the twitter search page result
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