#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::Redirect;
use rocket::response::NamedFile;

mod utils;

#[get("/favicon.ico")]
fn favicon() -> Option<NamedFile> {
    NamedFile::open("./static/favicon.co").ok()
}

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
    let command = utils::get_command_from_query_string(&cmd);

    match command {
        "mail" => Redirect::to("https://mail.google.com/"),
        "cal" => Redirect::to("https://calendar.google.com/"),
        "tw" => utils::twitter::construct_twitter_url(&cmd),
        "gh" => utils::github::construct_github_url(&cmd),
        _ => {
            // If no match, we search on Google
            let google_search = utils::google::construct_google_search_url(&cmd);

            Redirect::to(google_search)
        }
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index, search, favicon]).launch();
}
