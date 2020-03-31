#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::Redirect;
use rocket::http::RawStr;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
// rename cmd to query
#[get("/search?<cmd>")]
fn search(cmd: &RawStr) -> Redirect {
    println!("The cmd is: {}", cmd);
    // We need a way to match only on the cmd, without losing the rest of the query 
    // "tw something"
    // I need a function that will grab the command
    match cmd.as_str() {
        "mail" => Redirect::to("https://mail.google.com/"),
        "cal" => Redirect::to("https://calendar.google.com/"),
        _ => Redirect::to("https://google.com/")
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index, search]).launch();
}

// Write a function to construct twitter url 
// then maybe a function to construct twitter profile url
// and another to construct twitter search 

// fn construct_twitter_url(query: &RawStr) -> Redirect {
    // check the query
    // if it's only the cmd "tw"
    // send them to Twitter.com
    // if it contains the @symbol 
    // send them to that twitter profile
    // if it contains text but no @ symbol
    // send them to the twitter search page result
// }

fn get_command_from_query_string(query_string: &str) -> &str {
    // If it has a space, we know that it is more than the command
    if query_string.contains(" ") {
        // We need to this to know where to slice the string
        // TODO add note about why we need the unrap_or
        let index_of_space = query_string.find(" ").unwrap_or(0);
        return &query_string[..index_of_space];
    }

    return query_string;
}

fn construct_twitter_profile_url(profile: &str) -> String {
    // Make sure to remove the @
    // Twitter would redirect if we forgot to do this, but saves time 
    // not having to redirect
    format!("https://twitter.com/{}", profile.replace("@", ""))
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_construct_twitter_profile_url() {
      let fake_profile = "@jsjoeio";
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